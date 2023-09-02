use windows::core::{ComInterface, Error, Result};
use windows::Win32::Foundation::{HWND, RECT};
use windows::Win32::Graphics::Direct2D::{D2D1_BITMAP_OPTIONS_CANNOT_DRAW, D2D1_BITMAP_OPTIONS_TARGET, D2D1_BITMAP_PROPERTIES1, D2D1_DEBUG_LEVEL_INFORMATION, D2D1_DEVICE_CONTEXT_OPTIONS_NONE, D2D1_FACTORY_OPTIONS, D2D1_FACTORY_TYPE_SINGLE_THREADED, D2D1_UNIT_MODE_DIPS, D2D1CreateFactory, ID2D1DeviceContext, ID2D1Factory1};
use windows::Win32::Graphics::Direct2D::Common::{D2D1_ALPHA_MODE_PREMULTIPLIED, D2D1_PIXEL_FORMAT};
use windows::Win32::Graphics::Direct3D11::{D3D11_CREATE_DEVICE_BGRA_SUPPORT, D3D11_CREATE_DEVICE_DEBUG, D3D11_SDK_VERSION, D3D11CreateDevice, ID3D11Device};
use windows::Win32::Graphics::Direct3D::{D3D_DRIVER_TYPE, D3D_DRIVER_TYPE_HARDWARE, D3D_DRIVER_TYPE_WARP};
use windows::Win32::Graphics::DirectComposition::{DCompositionCreateDevice, IDCompositionDevice, IDCompositionTarget, IDCompositionVisual};
use windows::Win32::Graphics::DirectWrite::{DWRITE_FACTORY_TYPE_SHARED, DWriteCreateFactory, IDWriteFactory2};
use windows::Win32::Graphics::Dxgi::{CreateDXGIFactory1, DXGI_ERROR_UNSUPPORTED, DXGI_SWAP_CHAIN_DESC1, DXGI_SWAP_EFFECT_FLIP_SEQUENTIAL, DXGI_USAGE_RENDER_TARGET_OUTPUT, IDXGIDevice, IDXGIFactory2, IDXGISurface2, IDXGISwapChain1};
use windows::Win32::Graphics::Dxgi::Common::{DXGI_ALPHA_MODE_PREMULTIPLIED, DXGI_FORMAT_B8G8R8A8_UNORM, DXGI_SAMPLE_DESC};
use windows::Win32::UI::WindowsAndMessaging::GetClientRect;
use crate::BASE_DPI;

pub(crate) fn get_window_size(handle: HWND) -> Result<(u32, u32)> {
    let mut rect = RECT::default();
    unsafe {
        let bool = GetClientRect(handle, &mut rect);
        if !bool.is_ok() {
            return Err(Error::from_win32());
        }
    }

    Ok(((rect.right - rect.left) as u32, (rect.bottom - rect.top) as u32))
}

pub(crate) fn create_factory() -> Result<ID2D1Factory1> {
    let mut options = D2D1_FACTORY_OPTIONS::default();
    if cfg!(debug_assertions) {
        options.debugLevel = D2D1_DEBUG_LEVEL_INFORMATION;
    }

    unsafe {
        D2D1CreateFactory(
            D2D1_FACTORY_TYPE_SINGLE_THREADED,
            Some(&options),
        )
    }
}

pub(crate) fn create_dxgi_factory() -> Result<IDXGIFactory2> {
    unsafe {
        CreateDXGIFactory1()
    }
}

pub(crate) fn create_write_factory() -> Result<IDWriteFactory2> {
    unsafe {
        DWriteCreateFactory(DWRITE_FACTORY_TYPE_SHARED)
    }
}

pub(crate) fn get_dpi(factory: &ID2D1Factory1) -> Result<(f32, f32)> {
    let mut dpi_x = 0.0;
    let mut dpi_y = 0.0;

    unsafe {
        factory.GetDesktopDpi(&mut dpi_x, &mut dpi_y);
    }

    Ok((dpi_x, dpi_y))
}

// pub(crate) fn get_frequency() -> Result<i64> {
//     let mut frequency = 0;
//     unsafe {
//         QueryPerformanceFrequency(&mut frequency);
//     }
//     Ok(frequency)
// }

pub(crate) fn create_device() -> Result<ID3D11Device> {
    let mut result = create_device_with_type(D3D_DRIVER_TYPE_HARDWARE);

    if let Err(err) = &result {
        if err.code() == DXGI_ERROR_UNSUPPORTED {
            result = create_device_with_type(D3D_DRIVER_TYPE_WARP);
        }
    }

    result
}

pub(crate) fn create_device_with_type(drive_type: D3D_DRIVER_TYPE) -> Result<ID3D11Device> {
    let mut flags = D3D11_CREATE_DEVICE_BGRA_SUPPORT;
    if cfg!(debug_assertions) {
        flags |= D3D11_CREATE_DEVICE_DEBUG;
    }

    let mut device = None;

    unsafe {
        D3D11CreateDevice(
            None,
            drive_type,
            None,
            flags,
            None,
            D3D11_SDK_VERSION,
            Some(&mut device),
            None,
            None,
        ).map(|()| device.unwrap())
    }
}

pub(crate) fn create_render_context(factory: &ID2D1Factory1, device: &ID3D11Device) -> Result<ID2D1DeviceContext> {
    unsafe {
        let d2d_device = factory.CreateDevice(&device.cast::<IDXGIDevice>()?)?;
        let context = d2d_device.CreateDeviceContext(D2D1_DEVICE_CONTEXT_OPTIONS_NONE)?;
        context.SetUnitMode(D2D1_UNIT_MODE_DIPS);
        Ok(context)
    }
}

pub(crate) fn create_swap_chain(dxgi_factory: &IDXGIFactory2, device: &ID3D11Device, size: (u32, u32)) -> Result<IDXGISwapChain1> {
    let desc = DXGI_SWAP_CHAIN_DESC1 {
        Format: DXGI_FORMAT_B8G8R8A8_UNORM,
        SampleDesc: DXGI_SAMPLE_DESC {
            Count: 1,
            Quality: 0,
        },
        BufferUsage: DXGI_USAGE_RENDER_TARGET_OUTPUT,
        SwapEffect: DXGI_SWAP_EFFECT_FLIP_SEQUENTIAL,
        BufferCount: 2,
        AlphaMode: DXGI_ALPHA_MODE_PREMULTIPLIED,
        Width: size.0,
        Height: size.1,
        ..Default::default()
    };

    let dxgi_device: IDXGIDevice = device.cast::<IDXGIDevice>()?;

    unsafe {
        let result = dxgi_factory.CreateSwapChainForComposition(
            &dxgi_device,
            &desc,
            None,
        );
        if result.is_err() {
            return Err(Error::from_win32());
        }
        result
    }
}

pub(crate) fn create_swap_chain_bitmap(swap_chain: &IDXGISwapChain1, context: &ID2D1DeviceContext) -> Result<()> {
    let surface: IDXGISurface2 = unsafe {
        swap_chain.GetBuffer(0)?
    };

    let props = D2D1_BITMAP_PROPERTIES1 {
        pixelFormat: D2D1_PIXEL_FORMAT {
            format: DXGI_FORMAT_B8G8R8A8_UNORM,
            alphaMode: D2D1_ALPHA_MODE_PREMULTIPLIED,
        },
        dpiX: BASE_DPI,
        dpiY: BASE_DPI,
        bitmapOptions: D2D1_BITMAP_OPTIONS_TARGET | D2D1_BITMAP_OPTIONS_CANNOT_DRAW,
        ..Default::default()
    };

    unsafe {
        let bitmap = context.CreateBitmapFromDxgiSurface(&surface, Some(&props))?;
        context.SetTarget(&bitmap);
    }

    Ok(())
}

pub(crate) fn create_composition_device(device: &ID3D11Device) -> Result<IDCompositionDevice> {
    unsafe {
        DCompositionCreateDevice(&device.cast::<IDXGIDevice>()?)
    }
}

pub(crate) fn create_composition_target(device: &IDCompositionDevice, handle: HWND) -> Result<IDCompositionTarget> {
    unsafe {
        device.CreateTargetForHwnd(handle, true)
    }
}

pub(crate) fn create_visual(device: &IDCompositionDevice, comp_target: &IDCompositionTarget, swap_chain: &IDXGISwapChain1) -> Result<IDCompositionVisual> {
    let visual = unsafe {
        device.CreateVisual()
    }?;

    unsafe {
        visual.SetContent(swap_chain)?;
        comp_target.SetRoot(&visual)?;
        device.Commit()?;
    }

    Ok(visual)
}
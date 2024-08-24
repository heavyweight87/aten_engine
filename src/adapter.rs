use wgpu::{Instance, Surface, Adapter};

pub async fn get_adapter<'a>(surface: &'a wgpu::Surface) -> wgpu::Adapter {
    let instance = wgpu::Instance::default();
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            force_fallback_adapter: false,
            // Request an adapter which can render to our surface
            compatible_surface: Some(surface),
        })
        .await
        .expect("Failed to find an appropriate adapter");
    adapter
}
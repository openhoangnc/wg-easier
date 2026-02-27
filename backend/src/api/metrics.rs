use axum::response::IntoResponse;
use metrics_exporter_prometheus::PrometheusHandle;

pub async fn prometheus(
    axum::extract::Extension(handle): axum::extract::Extension<PrometheusHandle>,
) -> impl IntoResponse {
    handle.render()
}

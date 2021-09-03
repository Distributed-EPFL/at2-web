tonic::include_proto!("at2_ns");

#[cfg(feature = "server")]
impl<T: name_service_server::NameService> tonic::transport::NamedService
    for name_service_server::NameServiceServer<T>
{
    const NAME: &'static str = "at2_ns.NameService";
}

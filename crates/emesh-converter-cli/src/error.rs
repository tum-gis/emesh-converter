use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    EmeshConverterError(#[from] emesh_converter::Error),

    #[error(transparent)]
    EmeshError(#[from] emesh::Error),
    #[error(transparent)]
    EgraphicsError(#[from] egraphics::Error),
    #[error(transparent)]
    EgraphicsIoError(#[from] egraphics::io::Error),

    #[error(transparent)]
    StdIoError(#[from] std::io::Error),
}

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    EmeshError(#[from] emesh::Error),
    #[error(transparent)]
    EgraphicsError(#[from] egraphics::Error),
}

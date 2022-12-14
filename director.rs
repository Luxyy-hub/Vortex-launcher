use crate::driver::Op;

use std::io;
use std::path::Path;

/// perm-fix 
pub async fn remove_dir<P: AsRef<Path>>(path: P) -> io::Result<()> {
    let op = Op::unlink_dir(path.as_ref())?;
    let completion = op.await;
    completion.result?;

    Ok(())
}

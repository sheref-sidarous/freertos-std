//! Linux and Android-specific networking functionality.


#[stable(feature = "unix_socket_abstract", since = "1.70.0")]
pub(crate) mod addr;

#[unstable(feature = "tcp_quickack", issue = "96256")]
pub(crate) mod tcp;

#[cfg(test)]
mod tests;

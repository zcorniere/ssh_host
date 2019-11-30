use std::path::PathBuf;

/// this structure is used for storing the hostdata for easier storrage of the related information
///
/// The `name` field is the name of the remote.
///
/// The `address` field store the address of the remote hosts.
///
/// The `port` is for the remote port that ssh must use to connect.
///
/// The `private_key` is the path to the private_key to use.
pub struct SshHost {
   name: String,
   address: String,
   user: String,
   port: i64,
   private_key: Option<PathBuf>,
   reachable: bool,
}

impl SshHost {
    /// return a new struct `SshHost`
    ///
    /// # Examples
    ///
    /// ```
    /// # use ssh_host::ssh_host::SshHost;
    /// use std::path::PathBuf;
    ///
    /// let val = SshHost::new(String::from("hello"), String::from("127.0.0.1"), String::from("user"), 22, Some(PathBuf::from("private_key")));
    /// assert_eq!(val.get_name(), &String::from("hello"));
    /// assert_eq!(val.get_address(), &String::from("127.0.0.1"));
    /// assert_eq!(val.get_user(), &String::from("user"));
    /// assert_eq!(val.get_port(), &22);
    /// assert_eq!(val.get_private_key().as_ref().unwrap(), &PathBuf::from("private_key"));
    /// assert_eq!(val.is_reachable(), &true);
    /// assert_eq!(val.get_url(), String::from("127.0.0.1:22"));
    /// ```
    pub fn new(name: String, address: String, user: String, port: i64, private_key: Option<PathBuf>) -> SshHost {
        SshHost {
            name: name.clone(),
            address: address.clone(),
            user: user.clone(),
            port: port.clone(),
            private_key: private_key.clone(),
            reachable: true,
        }
    }
    /// return the name of the remote
    ///
    /// # Examples
    ///
    /// ```
    /// # use ssh_host::ssh_host::SshHost;
    /// use std::path::PathBuf;
    ///
    /// let val = SshHost::new(String::from("hello"), String::from("127.0.0.1"), String::from("user"), 22, Some(PathBuf::from("private_key")));
    /// assert_eq!(val.get_name(), &String::from("hello"));
    /// ```
    pub fn get_name(&self) -> &String {
        &self.name
    }
    /// set the name of the remote
    ///
    /// # Examples
    ///
    /// ```
    /// # use ssh_host::ssh_host::SshHost;
    /// use std::path::PathBuf;
    ///
    /// let mut val = SshHost::new(String::from("hello"), String::from("127.0.0.1"), String::from("user"), 22, Some(PathBuf::from("private_key")));
    /// assert_eq!(val.get_name(), &String::from("hello"));
    /// val.set_name(String::from("foobar"));
    /// assert_eq!(val.get_name(), &String::from("foobar"));
    /// ```
    pub fn set_name(&mut self, new_name: String) {
        self.name = new_name
    }
    /// return the address of the remote
    ///
    /// # Examples
    ///
    /// ```
    /// # use ssh_host::ssh_host::SshHost;
    /// use std::path::PathBuf;
    ///
    /// let val = SshHost::new(String::from("hello"), String::from("127.0.0.1"), String::from("user"), 22, Some(PathBuf::from("private_key")));
    /// assert_eq!(val.get_address(), &String::from("127.0.0.1"));
    /// ```
    pub fn get_address(&self) -> &String {
        &self.address
    }
    /// set the address of the remote
    ///
    /// # Examples
    ///
    /// ```
    /// # use ssh_host::ssh_host::SshHost;
    /// use std::path::PathBuf;
    ///
    /// let mut val = SshHost::new(String::from("hello"), String::from("127.0.0.2"), String::from("user"), 22, Some(PathBuf::from("private_key")));
    /// assert_eq!(val.get_address(), &String::from("127.0.0.2"));
    /// val.set_address(String::from("127.0.0.1"));
    /// assert_eq!(val.get_address(), &String::from("127.0.0.1"));
    /// ```
    pub fn set_address(&mut self, new_address: String) {
        self.address = new_address;
    }
    /// return the user of the remote
    ///
    /// # Examples
    ///
    /// ```
    /// # use ssh_host::ssh_host::SshHost;
    /// use std::path::PathBuf;
    ///
    /// let val = SshHost::new(String::from("hello"), String::from("127.0.0.1"), String::from("user"), 22, Some(PathBuf::from("private_key")));
    /// assert_eq!(val.get_user(), &String::from("user"));
    /// ```
    pub fn get_user(&self) -> &String {
        &self.user
    }
    /// set the address of the remote
    ///
    /// # Examples
    ///
    /// ```
    /// # use ssh_host::ssh_host::SshHost;
    /// use std::path::PathBuf;
    ///
    /// let mut val = SshHost::new(String::from("hello"), String::from("127.0.0.1"), String::from("user"), 22, Some(PathBuf::from("private_key")));
    /// assert_eq!(val.get_user(), &String::from("user"));
    /// val.set_user(String::from("root"));
    /// assert_eq!(val.get_user(), &String::from("root"));
    /// ```
    pub fn set_user(&mut self, new_user: String) {
        self.user = new_user;
    }
    /// return the address of the remote
    ///
    /// # Examples
    ///
    /// ```
    /// # use ssh_host::ssh_host::SshHost;
    /// use std::path::PathBuf;
    ///
    /// let val = SshHost::new(String::from("hello"), String::from("127.0.0.1"), String::from("user"), 22, Some(PathBuf::from("private_key")));
    /// assert_eq!(val.get_port(), &22);
    /// ```
    pub fn get_port(&self) -> &i64 {
        &self.port
    }
    /// set the port of the remote
    ///
    /// # Examples
    ///
    /// ```
    /// # use ssh_host::ssh_host::SshHost;
    /// use std::path::PathBuf;
    ///
    /// let mut val = SshHost::new(String::from("hello"), String::from("127.0.0.1"), String::from("user"), 22, Some(PathBuf::from("private_key")));
    /// assert_eq!(val.get_port(), &22);
    /// val.set_port(2222);
    /// assert_eq!(val.get_port(), &2222);
    /// ```
    pub fn set_port(&mut self, new_port: i64) {
        self.port = new_port;
    }
    /// return the address of the remote
    ///
    /// # Examples
    ///
    /// ```
    /// # use ssh_host::ssh_host::SshHost;
    /// use std::path::PathBuf;
    ///
    /// let val = SshHost::new(String::from("hello"), String::from("127.0.0.1"), String::from("user"), 22, Some(PathBuf::from("private_key")));
    /// assert_eq!(val.get_private_key().as_ref().unwrap(), &PathBuf::from("private_key"));
    /// ```
    pub fn get_private_key(&self) -> &Option<PathBuf> {
        &self.private_key
    }
    /// set the the private_key of the remote
    ///
    /// # Examples
    ///
    /// ```
    /// # use ssh_host::ssh_host::SshHost;
    /// use std::path::PathBuf;
    ///
    /// let mut val = SshHost::new(String::from("hello"), String::from("127.0.0.1"), String::from("user"), 22, Some(PathBuf::from("private_key")));
    /// assert_eq!(val.get_private_key().as_ref().unwrap(), &PathBuf::from("private_key"));
    /// val.set_private_key(Some(PathBuf::from("private_key_2")));
    /// assert_eq!(val.get_private_key().as_ref().unwrap(), &PathBuf::from("private_key_2"));
    /// ```
    pub fn set_private_key(&mut self, new_key: Option<PathBuf>) {
        self.private_key = new_key;
    }
    
    /// return if the remote hosts is available or not
    ///
    /// # Examples
    ///
    /// ```
    /// # use ssh_host::ssh_host::SshHost;
    /// use std::path::PathBuf;
    ///
    /// let val = SshHost::new(String::from("hello"), String::from("127.0.0.1"), String::from("user"), 22, Some(PathBuf::from("private_key")));
    /// assert_eq!(val.is_reachable(), &true);
    /// ```
    pub fn is_reachable(&self) -> &bool {
        &self.reachable
    }
    
    /// set the remote value
    ///
    /// # Examples
    ///
    /// ```
    /// # use ssh_host::ssh_host::SshHost;
    /// use std::path::PathBuf;
    ///
    /// let mut val = SshHost::new(String::from("hello"), String::from("127.0.0.1"), String::from("user"), 22, Some(PathBuf::from("private_key")));
    /// assert_eq!(val.is_reachable(), &true);
    /// val.set_reachable(false);
    /// assert_eq!(val.is_reachable(), &false);
    /// ```
    pub fn set_reachable(&mut self, new_val: bool) {
        self.reachable = new_val;
    }
    
    /// return the url of the hosts
    ///
    /// It will be in the form of `address:port` like `127.0.0.1:22`
    ///
    /// # Examples
    ///
    /// ```
    /// # use ssh_host::ssh_host::SshHost;
    /// use std::path::PathBuf;
    ///
    /// let val = SshHost::new(String::from("hello"), String::from("127.0.0.1"), String::from("user"), 22, Some(PathBuf::from("private_key")));
    /// assert_eq!(val.get_url(), String::from("127.0.0.1:22"));
    /// ```
    pub fn get_url(&self) -> String {
        let mut val = String::from(&self.address);
        val.push_str(":");
        val.push_str(&self.port.to_string());
        return val;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_method_new() {
        let val = SshHost::new(String::from("hello"), String::from("127.0.0.1"), String::from("user"), 22, Some(PathBuf::from("private_key")));
        assert_eq!(val.get_address(), &String::from("127.0.0.1"));
        assert_eq!(val.get_port(), &22);
        assert_eq!(val.get_private_key().as_ref().unwrap(), &PathBuf::from("private_key"));
        assert_eq!(val.get_name(), &String::from("hello"));
    }
    #[test]
    fn test_get_name() {
        let val = SshHost::new(String::from("hello"), String::from("127.0.0.1"), String::from("user"), 22, Some(PathBuf::from("private_key")));
        assert_eq!(val.get_name(), &String::from("hello"));
    }
    #[test]
    fn test_set_name() {
        let mut val = SshHost::new(String::from("hello"), String::from("127.0.0.1"), String::from("user"), 22, Some(PathBuf::from("private_key")));
        assert_eq!(val.get_name(), &String::from("hello"));
        val.set_name(String::from("foobar"));
        assert_eq!(val.get_name(), &String::from("foobar"));
    }
    #[test]
    fn test_get_address() {
        let val = SshHost::new(String::from("hello"), String::from("127.0.0.1"), String::from("user"), 22, Some(PathBuf::from("private_key")));
        assert_eq!(val.get_address(), &String::from("127.0.0.1"));
    }
    #[test]
    fn test_set_address() {
        let mut val = SshHost::new(String::from("hello"), String::from("127.0.0.2"), String::from("user"), 22, Some(PathBuf::from("private_key")));
        assert_eq!(val.get_address(), &String::from("127.0.0.2"));
        val.set_address(String::from("127.0.0.1"));
        assert_eq!(val.get_address(), &String::from("127.0.0.1"));
    }
    #[test]
    fn test_get_user() {
        let val = SshHost::new(String::from("hello"), String::from("127.0.0.1"), String::from("user"), 22, Some(PathBuf::from("private_key")));
        assert_eq!(val.get_user(), &String::from("user"));
    }
    #[test]
    fn test_set_user() {
        let mut val = SshHost::new(String::from("hello"), String::from("127.0.0.1"), String::from("user"), 22, Some(PathBuf::from("private_key")));
        assert_eq!(val.get_user(), &String::from("user"));
        val.set_user(String::from("root"));
        assert_eq!(val.get_user(), &String::from("root"));
    }
    #[test]
    fn test_get_port() {
        let val = SshHost::new(String::from("hello"), String::from("127.0.0.1"), String::from("user"), 22, Some(PathBuf::from("private_key")));
        assert_eq!(val.get_port(), &22);
    }
    #[test]
    fn test_set_port() {
        let mut val = SshHost::new(String::from("hello"), String::from("127.0.0.1"), String::from("user"), 22, Some(PathBuf::from("private_key")));
        assert_eq!(val.get_port(), &22);
        val.set_port(2222);
        assert_eq!(val.get_port(), &2222);
    }
    #[test]
    fn test_get_private_key() {
        let val = SshHost::new(String::from("hello"), String::from("127.0.0.1"), String::from("user"), 22, Some(PathBuf::from("private_key")));
        assert_eq!(val.get_private_key().as_ref().unwrap(), &PathBuf::from("private_key"));
    }
    #[test]
    fn test_set_private_key() {
        let mut val = SshHost::new(String::from("hello"), String::from("127.0.0.1"), String::from("user"), 22, Some(PathBuf::from("private_key")));
        assert_eq!(val.get_private_key().as_ref().unwrap(), &PathBuf::from("private_key"));
        val.set_private_key(Some(PathBuf::from("private_key_2")));
        assert_eq!(val.get_private_key().as_ref().unwrap(), &PathBuf::from("private_key_2"));
    }
    #[test]
    fn test_is_reachable() {
        let val = SshHost::new(String::from("hello"), String::from("127.0.0.1"), String::from("user"), 22, Some(PathBuf::from("private_key")));
        assert_eq!(val.is_reachable(), &true);
    }
    #[test]
    fn test_set_reachable() {
        let mut val = SshHost::new(String::from("hello"), String::from("127.0.0.1"), String::from("user"), 22, Some(PathBuf::from("private_key")));
        assert_eq!(val.is_reachable(), &true);
        val.set_reachable(false);
        assert_eq!(val.is_reachable(), &false);
    }
    #[test]
    fn test_get_url() {
        let val = SshHost::new(String::from("hello"), String::from("127.0.0.1"), String::from("user"), 22, Some(PathBuf::from("private_key")));
        assert_eq!(val.get_url(), String::from("127.0.0.1:22"));
    }
}

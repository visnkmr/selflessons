Since tns_daemon makes use of http connection it takes ownership of a port. Since one port can only be owned by one process at a time muliple instances will not be allowed.

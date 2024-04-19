
Cloned from https://github.com/fluencelabs/curl-effector with minor tweaks: 

added method curl_post_binary for posting content of data_vault_path with the --data-binary path, so that carriage returns and white spaces are not removed from content of file
which is relevant when uploading files as multipart, for example with Pinata. 

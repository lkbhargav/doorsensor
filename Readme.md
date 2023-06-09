Raspberry pi model 3 B pinouts: https://www.raspberrypi.com/documentation/computers/raspberry-pi.html

SQLite3 commands:
`sqlite3 <db_file_path>/<db_name>.db`

`.schema` => prints out the table schema
`.help` => you know what it is for ;-)
`.exit` => does what it says
`.output <path>/<filename>.sql` => sets the path for the consequent commands output to be recorded in 
`.dump` prints out the table schema and all of the data with insert statements

Sample query:
`select id, datetime(timestamp, 'localtime'), notes from logs where is_door_open=1;`

Deployment steps:
1. Push the changes to the Repo (git push origin master)
2. Pull it on the server (git pull origin master) (doorsensor - 188)
3. Build the project and move the binary to `/usr/local/bin/`
4. Update the environment variables if necessary from the Supervisor config
5. Run `reread` and `update` commands on the supervisorctl

**Note**: `Cross` doesn't work since the build requires some dependecies on openssl which doesn't play well. For now the config is still here. Also, Raspberry pi should have both `sqlite3` and `libsqlite3-dev` packages installed or there will be compilation errors
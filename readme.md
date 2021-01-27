# instructions ubuntu

First install [elm](https://guide.elm-lang.org/install/elm.html) and [rust](https://www.rust-lang.org/tools/install). For rust you will need to use the nightly version. To do this run following command in the root folder of this project.

```
rustup override set nightly
```

Starting in the root folder of this project, run following commands to build the project.

```
cd client
elm make src/Main.elm --output=script.js
cd ../server
cargo build --release
```

Now we have an executable we give it permissions and run it as a service.

```
cd ..
sudo chmod +x /server/target/release/remote_controller
cd /etc/systemd/system
sudo nano remote_controller.service
```

Write following to the file and save with ctrl+x and y.

```
[Unit]
Description=remote controller

[Service]
Type=simple
WorkingDirectory=/home/tsrapnik/projects/remote_controller/pc_server
ExecStart=/home/tsrapnik/projects/remote_controller/pc_server/target/release/remote_controller

[Install]
WantedBy=multi-user.target
```

Of course replace path_to_your_project_folder by the actual path to your project.

We need to give the file certain permissions.

```
sudo chmod 644 /etc/systemd/system/remote_controller.service
```

Now we only need to start the service.

```
sudo systemctl start remote_controller
```

You can check if it is running with following command.

```
sudo systemctl status remote_controller
```

Stop or restart with following.

```
sudo systemctl stop remote_controller
sudo systemctl restart remote_controller
```

If you want to automatically start the service at boot use following.

```
sudo systemctl enable remote_controller
```

When running ubuntu use following command to allow the server through the firewall.

```
sudo ufw allow 80/tcp
```
# rsync_ssh

善！通过配置文件从实验室的服务器上同步回自己的数据！

## 配置文件

配置文件（YAML格式）放置于用户目录的`.rsync_ssh`目录下，文件名以`.yml`做结尾。包含以下属性：

- username: SSH用户名
- hostname: SSH主机名
- port: 可选，远程主机的SSH端口号
- remote_path: 远程路径
- local_path: 本地路径
- exclude: 要排除的路径

> 身份验证请通过公钥等非交互式登录方式进行

## 单次同步

使用`rsync_ssh -c <CONFIG_NAME> one-shot`进行单次同步，例如配置文件`.rsync_ssh/lily.yml`对应的命令是`rsync_ssh -c lily one-shot`

## 持续同步

使用`rsync_ssh -c <CONFIG_NAME> daemon --interval <INTERVAL_SECS>`进行持续同步，每次同步后，会根据`interval`指定的秒数进行等待后再次进行同步，例如间隔十分钟同步`lily`数据，命令为：`rsync_ssh -c lily daemon --interval 600`

## 创建配置文件（CLI）

可以通过命令行创建配置文件。例如创建名为jack的配置：

```bash
rsync_ssh -c jack \
init \
--username xxx \
--hostname jack.local \
--port 4230 \
--remote-path ./ \
--local-path /data/jack
```

`exclude`和`port`参数都是可选的。

## systemd配置样例

### 使用内置持续同步

```
#~/.config/systemd/user/rsync_ssh@.service 
[Unit]
Description=Auto sync data with remote server by rsync over ssh

[Service]
ExecStart=/home/hexiang/.local/bin/rsync_ssh -c %i daemon --interval 600

[Install]
WantedBy=default.target
```

### 使用systemd计时器

```
#~/.config/systemd/user/rsync_ssh@.service 
[Unit]
Description=Auto sync data with remote server by rsync over ssh

[Service]
Type=oneshot
ExecStart=/home/hexiang/.local/bin/rsync_ssh -c %i one-shot
```

```
#~/.config/systemd/user/rsync_ssh@.timer
[Unit]
Description=Timer for auto sync data with remote server by rsync over ssh

[Timer]
OnCalendar=hourly
RandomizedDelaySec=15min
Persistent=true

[Install]
WantedBy=default.target
```

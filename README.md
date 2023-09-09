# Re: TTSCat Edge TTS 转发服务器

配合 https://github.com/Elepover/RE-TTSCat 使用

## 使用方法

设置 Re: TTSCat 的高级选项：

* TTS 引擎: `用户自定义引擎`
* 自定义 TTS 引擎 URL: `http://127.0.0.1:23456/tts?text=$TTSTEXT`
* 自定义 TTS 引擎请求方式: `GET`

然后运行服务器即可。

使用命令行可以添加一个额外参数，表示监听的地址和端口号，例如 `127.0.0.1:8000` 或 `0.0.0.0:80` 等

### 使用不同的声音模型

修改自定义 TTS 引擎 URL，添加 voice 参数: `http://127.0.0.1:23456/tts?voice=zh-CN-XiaoxiaoNeural&text=$TTSTEXT`

常用列表

* zh-CN-XiaoxiaoNeural
* zh-CN-XiaoyiNeural
* zh-CN-YunjianNeural
* zh-CN-YunxiNeural
* zh-CN-YunxiaNeural
* zh-CN-YunyangNeural
* zh-CN-liaoning-XiaobeiNeural
* zh-CN-shaanxi-XiaoniNeural
* zh-HK-HiuGaaiNeural
* zh-HK-HiuMaanNeural
* zh-HK-WanLungNeural
* zh-TW-HsiaoChenNeural
* zh-TW-YunJheNeural
* zh-TW-HsiaoYuNeural
* en-US-AriaNeural
* en-US-AnaNeural
* en-US-ChristopherNeural
* en-US-EricNeural
* en-US-GuyNeural
* en-US-JennyNeural
* en-US-MichelleNeural
* en-US-RogerNeural
* en-US-SteffanNeural

voice 的完整列表可以参考 https://speech.platform.bing.com/consumer/speech/synthesize/readaloud/voices/list?trustedclienttoken=6A5AA1D4EAFF4E9FB37E23D68491D6F4 里面的 ShortName 字段。

## 说明

本 HTTP 服务器实现较简单，使用多线程模型，没有使用线程池，仅适用于低并发。

## LICENSE

MIT License

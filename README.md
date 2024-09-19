# VRChat SystemInfo OSC

> [!WARNING]  
> 一部機能は開発中につき不安定な場合があります。  
> また、本システムのメインターゲットはLinuxなため、Windows及びMacOSでは正しく動作しない可能性があります。

## 概要

VRChatのChatboxにCPU使用率や空きメモリ容量などを表示するOSCツールです。
主にLinux用として作成していますが、Windows、Mac OSでも一部機能は利用可能です。

## 設定

### 設定方法

Settings/settings.jsonを作成し、json形式で設定を記述します。下記はsettings.jsonのデフォルト値を入力した完全なサンプルです。

``` json  
{
    "log_level": "warn",
    "osc_port": {
        "send_to_vrchat": 9000,
        "recv_from_vrchat": 9001
    },
    "send_chatbox": {
        "presets": [
            "CPU Usage: ${cpu_usage}%\nGPU Usage: ${gpu_usage}%\nMem: ${used_mem}/${total_mem}"
        ],
        "preset_key": "/avatar/parameters/SysinfoPreset"
    }
}

```

設定は省略することができます。省略した場合はデフォルト値に設定されます。

### 各設定の詳細情報

- log_level: ログの出力レベルを指定します。デフォルトは`warn`です。
- osc_port:　OSCの情報をVRChatと受け渡しするために必要な設定です。
  - send_to_vrchat: 送信先のVRChatのポートを指定します。デフォルトは9000です。
  - recv_from_vrchat: VRChatからの情報を受け取るポートを指定します。デフォルトは9001です。
- send_chatbox
  - presets: プリセットの一覧です。ここに表示したい文字を入れておくとVRChatに送られます。
  - preset_key: プリセットを切り替えるためのパラメーターです。VRChat上で表示を切り替えるために使います。

### 設定可能な変数

presetsで送信する情報には変数を含めることができます。下記はその変数の一覧です。  

|変数名|説明|
|-|-|
|os_name|OSの名前です。|
|cpu_usage|CPU使用率です。0~100の％表記。|
|gpu_usage|GPU使用率です。0~100の％表記。|
|mem_usage|メモリの使用率です。0~100の％表記。|
|vram_usage|VRAMの使用率です。0~100の％表記。【Linux以外未実装】|
|cpu_temp|CPUの温度です。【未実装】|
|gpu_temp|GPUの温度です。【未実装】|
|total_mem|メモリの最大容量です。|
|used_mem|メモリの使用量です。|
|total_vram|VRAMの最大容量です。【Linux以外未実装】|
|used_vram|VRAMの使用量です。【Linux以外未実装】|
|||

変数は`${`と`}`で囲むことで送信時に変換されます。半角と全角を間違えないようにしてください。

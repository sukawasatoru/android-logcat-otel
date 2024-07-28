android-logcat-otel
===================

Convert and post logs as OTel logs collected by `adb logcat`

Usage
-----

### CLI ###

```bash
./android-logcat-otel --logs-endpoint "http://localhost:4318/otlp/v1/logs"
```

### Grafana Dashboard ###

```
{service_namespace="android-logcat-otel", service_name="android-logcat-otel"}
| event_name = `device.app.logcat`
| timestamp>=$__from and timestamp<=$__to
| line_format "[{{unixToTime .timestamp | date `2006-01-02 15:04:05.000`}}] {{__line__}}"
```

```
{service_namespace="android-logcat-otel", service_name="android-logcat-otel"}
| event_name = `device.app.logcat`
| timestamp>=$__from and timestamp<=$__to
| tag=`ActivityManager`
|= `Start proc` or `Killing`
| line_format "[unixToTime .timestamp | date `2006-01-02 15:04:05.000`}} JST] {{__line__}}"
```

LICENSE
-------

```
   Copyright 2024 sukawasatoru

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
```

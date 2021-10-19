# TODO list
- 如何转成同步
- 补齐文档
- api 的函数名要尽量简写
- callback 函数 以及 对应的 event 处理流程
- set_data 这种要支持直接传入一个具体类型，然后通过一些可配置的序列化规则（例如：json）存入 data 字段
- get_data 这种要支持直接返回一个具体类型，然后通过一些可配置的序列化规则（例如：json）反序列化成对象结果
- 创建和删除 API 要支持递归
- Perms 需要更有意义的类型，并且转成 String 后需要更容易识别的，比如 CRWDA 这种，或者 Read Write 这种
- version 字段需要换成枚举
- addWatch 使用引用当场触发解决了只触发一次的情况，有没有更好的写法？
- 提供命令行工具解析 ZK 的快照文件和日志文件，并可以修改

# java client api
- [x] create
- [x] delete
- [x] getData
- [x] setData
- [x] exists
- [x] getChildren
- [x] getAllChildrenNumber
- [x] getEphemerals
- [x] getChildren2
- [x] getState
- [x] getSessionId
- [x] getSessionTimeout
- [x] getACL
- [x] setACL
- [x] addWatch
- [ ] removeWatches
- [ ] removeAllWatches
- [ ] getConfig
- [ ] updateServerList
- [ ] multi
- [ ] transaction
- [ ] sync

# java client async api
- [ ] create
- [ ] delete
- [ ] getData
- [ ] setData
- [ ] exists
- [ ] updateServerList
- [ ] getSessionId
- [ ] getSessionTimeout
- [ ] multi
- [ ] transaction
- [ ] getConfig
- [ ] getACL
- [ ] getChildren
- [ ] getAllChildrenNumber
- [ ] getEphemerals
- [ ] sync
- [ ] removeWatches
- [ ] removeAllWatches
- [ ] addWatch
- [ ] getState


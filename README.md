# Observer 模式

在观察者模式（Observer Pattern）中，一个目标对象管理所有相依于它的观察者对象，并且在它本身的状态改变时主动发出通知。这通常透过呼叫各观察者所提供的方法来实现。此种模式通常被用来实时事件处理系统。

观察者模式有四个角色：

- `ISubject`：抽象被观察者，将所有的观察这放入集合中，提供以下接口：
  - `add_observer()`：添加
  - `remove_observer()`：删除
  - `notify_observers()`：通知
- `Subject`：具体被观察者，将有关状态存入具体观察者对象，在具体主题的内部状态发生改变时，给所有注册过的观察者发生通知
- `IObserver`：抽象观察者，提供「更新接口」的定义
- `Observer`：具体观察者，实现抽象观察者定义的「更新接口」，以便在的到主题更改通知时更新自身的状态

![Observer](https://upload.wikimedia.org/wikipedia/commons/e/e2/Observer-pattern-class-diagram.png)

观察者模式也常被称为「发布/订阅（Publish / Subscribe）」模式。

# 参考资料

- [观察者模式-Wikipedia](https://zh.wikipedia.org/zh-cn/%E8%A7%82%E5%AF%9F%E8%80%85%E6%A8%A1%E5%BC%8F)
- [Rust与观察者模式](https://www.bilibili.com/video/BV1gb4y1y7CM?from=search&seid=3738269869142051655&spm_id_from=333.337.0.0)
- [lpxxn的示例代码](https://github.com/lpxxn/rust-design-pattern/blob/master/behavioral/observer.rs)
- [yukihir0的示例代码](https://github.com/yukihir0/rust_design_pattern/blob/master/observer/src/main.rs)
- [Observer](https://refactoring.guru/design-patterns/observer)

---

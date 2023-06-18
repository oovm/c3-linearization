Title
=====


## 子类

Valkyrie 遵从结构化类型, 也称静态鸭子类型.

如果类 A 中所有的接口另一个类 B 全部都有, 那么我们称 `A is a B`.

和 A, B 是否是继承实现的完全没有关系, 无论 B 是传统的继承类型, 还是继承的原始类型, 还是匿名类型都成立.

注意只检查静态类型, 运行期的增减并不影响 `is-a` 关系.

作为替代, 运行期使用 `is_instance` 检查, 且此时这要求 A, B 都实现 RTTI.

## 类继承

生成子类最常规的方法就是继承.

类继承分为实继承和虚继承
- 实继承: 父类存为成员变量, 接口不需要重新实现, 调用直接转发给父类即可.
- 虚继承: 父类不存在, 接口需要重新实现.

实继承可以带访问修饰符来决定成员变量的访问权限
- private: class level, 只能类内部访问
- protect: module level, 只能模块内部访问
- internal: package level, 只能包内部访问
- public: 可以任意访问

举个例子, 如下继承关系

```scala
class K1(A, B) { }
class K2(virtual A, public B) { };
```

宏展开后为

```scala
class K1 {
    private _a: A
    private _b: B
}

class K2 {
    public b: B
}
```

## 方法解析顺序

多继承, 特别是菱形继承的情况下, 需要解析方法的调用顺序.

解析需要遵从三个原则 (Consistent)

- 扩展一致性原则
- 局部优先原则
- 单调性原则

满足这三个原则的算法就是 C3 线性化算法, 解析结果称为方法解析序(MRO, Method Resolution Order).


```scala
class A(object) {}
class B(object) {}
class C(object) {}
class D(object) {}
class E(object) {}
class K1(C, A, B) {}
class K2(B, D, E) {}
class K3(A, D) {}
class Z(K1, K3, K2) {}
```

![](https://upload.wikimedia.org/wikipedia/commons/4/47/C3_linearization_example.svg)

其 mro 应为 `[Z, K1, C, K3, A, K2, B, D, E, object]`

## 方法属性

在类型层面, 简单起见, 我们把字段属性看成方法的一种, 我们不区分以下两者:

```scala
field: T
field(): T
```

实继承的情况下, 方法有如下几种修饰词
- inherit: 不写, 默认转发父类同名方法
- virtual: 无法调用, 子类必须重写, 除非子类是虚基类
- override: 重写父类方法, 除非父类方法是 final 方法
- final: 原则上禁止重写
- extend: 重写父类方法, 且调用父类方法
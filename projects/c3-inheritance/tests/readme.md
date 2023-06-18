## Tests

```scala

async fun coro(in: str) -> int {
    await inner(in)
}
```

```scala
effect structure Logging {
    trace(msg: str, span: Span): Unit {

    }
    debug(msg: str, span: Span): Unit
    print(msg: str, span: Span): Unit
    error(msg: str, span: Span): Unit
    fatal(msg: str, span: Span): Unit
    event(msg: str, level: Level, span: Span): Unit {
        let log = LogEvent::new(msg, Level::Trace, span);
        log.write_to(Stdout::new())
    }
}

structure LogEvent {
    msg: str
    level: Level
    span: Span
    
    
    write(writer: Writer) {
        writer.write(msg)
    }
}



effect structure DividedByZero {
    apply(denominator: int): int
}

fun div2(a: int, b: int) -> int {
    let out = if (b == 0) {
        let out = raise DividedByZero(a)
        out * 2
    }
    else {
        a / b
    }
}

try {
    div2(0, 0)
    div2(1, 0)
}
.catch {
    case DividedByZero(a):
        if a == 0 {
            return 999
        }
        else {
            resume a
        }
}
```


```scala
class Div2 {
    a: int;
    b: int;
    ret: int
}


fun div2(a: int, b: int, error: DividedByZero?) -> int {
    let out = if (b == 0) {
        let error = DividedByZero(a)
        let out = error
        out * 2
    }
    else {
        a / b
    }
}

let error = null
div2(0, 0, error)
div2(1, 0, error)

if error != null {
    if error.a == 0 {
        return 999
    }
    else {
        resume error.a
    }
}
```

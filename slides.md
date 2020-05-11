## Slajdy i przykłady

https://github.com/hashedone/borrow-checker

----

## Pamięć

### C++

* Wysoka wydajność
* Wycieki pamięci to przeszłość - smart pointer
* Użycie zwolnionej pamięci
* Dereferencja nieprawidłowego wskaźnika
  * Pusty wskaźnik
  * Indeksowanie poza zakresem
* Wyścigi danych

---

## Pamięć

### Języki zarządzane

* Niższa wydajność (teoretycznie)
* Wycieki pamięci właściwie nie istnieją
* Użycie zwolnionej pamięci - niemożliwe
* Dereferencja nieprawidłowego wskaźnika?
  * `NullPointerException`...?
* Wyścigi danych możliwe do uniknięcia

---

> Około 70% wszystkich błędów w produktach Microsoftu adresowanych
> w aktualizacjach bezpieczeństwa każdego roku, to problemy związane
> z bezpieczeństwem pamięci.

Catalin Cimpanu, MS security engineer, ZDNet, 2019-02-11

---

RustFest Barcelone - Sebastian Fernandex & Ryan Levick: R-Evolution: A Story of Rust Adoption

https://www.youtube.com/watch?v=o01QmYVluSw

---

## Bezpiecznie == Wolno?

---

## Rust

* Performance - "Zero cost abstraction"
* Reliability - Bezpieczeństwo pamięci i wątków
* Productivity - Bardzo dobra dokumentacja, przyjazny kompilator

---

## Rust

### ZCA + Memory safety + Thread safety?

# WTF?

---

## Darmowe abstrakcje

### C++ Move Semantics

```cpp
// semantical_move/main.cpp
class DestroyMe {
public:
    DestroyMe() {}

    DestroyMe(DestroyMe && orig) {
        std::cout << "Moved\n";
    }

    ~DestroyMe() {
        std::cout << "Destroyed\n";
    }
};
```

---

### Move Semantics

```cpp
// semantical_move/main.cpp
void foo(DestroyMe dm) {
    std::cout << "foo called\n";
}

int main() {
    auto dm = DestroyMe {};
    foo(std::move(dm));
    foo(std::move(dm));
}
```
```bash
Moved
foo called
Destroyed
Moved
foo called
Destroyed
Destroyed
```

---

### Rust - Semantical move

```rust
// semantical_move/main.rs
struct DestroyMe;

impl Drop for DestroyMe {
    fn drop(&mut self) {
        println!("Destroyed");
    }
}
```

---

### Semantical move

```rust
// semantical_move/main.rs
fn foo(dm: DestroyMe) {
    println!("foo called");
}

fn main() {
    let dm = DestroyMe;
    foo(dm);
    foo(dm);
}
```

```bash
error[E0382]: use of moved value: `dm`
  --> main.rs:16:9
   |
14 |     let dm = DestroyMe;
   |         -- move occurs because `dm` has type `DestroyMe`,
   |            which does not implement the `Copy` trait
15 |     foo(dm);
   |         -- value moved here
16 |     foo(dm);
   |         ^^ value used here after move
```

---

### Semantical move

```rust
// semantical_move/main1.rs
fn main() {
    let dm = DestroyMe;
    foo(dm);
}
```

```bash
foo called
Destroyed
```

----

## Rust Borrows

### Trochę jak referencje...

```rust
// borrows/1.rs
fn main() {
    let mut number: i32 = 41;
    let borrow: &i32 = &number;
    println!("Number: {}, borrow: {}", number, borrow);
    let mut_borrow: &mut i32 = &mut number;
    *mut_borrow += 1;
    println!("Answer: {}, mut_borrow: {}", number, mut_borrow);
}
```

```bash
Number: 41, borrow: 41
Answer: 42
```

---

## Rust Borrows

### ... trochę jak wskaźniki

```rust
// borrows/2.rs
fn main() {
    let number = 41;
    let answer = 42;
    let mut borrow = &number;
    println!("Borrow: {}", borrow);
    borrow = &answer;
    println!("Borrow: {}", borrow);
}
```

```bash
Borrow: 41
Borrow: 42
```

---

## Rust Borrows

### A jednak coś nowego

```rust
// borrows/3.rs
fn main() {
    let mut number = 41;
    let borrow = &number;
    let mut_borrow = &mut borrow;
    *mut_borrow += 1;
    println!("Borrow: {}", borrow);
}
```

---

### Rust Borrows

```bash
error[E0502]: cannot borrow `number` as mutable because it
              is also borrowed as immutable
 --> 3.rs:4:22
  |
3 |     let borrow = &number;
  |                  ------- immutable borrow occurs here
4 |     let mut_borrow = &mut number;
  |                      ^^^^^^^^^^^ mutable borrow occurs here
5 |     *mut_borrow += 1;
6 |     println!("Borrow: {}", borrow);
  |                            ------ immutable borrow later
  |                                   used here
```

---

## Zasady pożyczania

* Jednocześnie można pożyczyć obiekt współdzielnie (shared borrow) wielokrotnie LUB
* Pożyczyć go raz mutowalnie (mutable borrow)

### Nigdy jednoczesnie

---

## Zasady pożyczania

* Nie może jednocześnie istnieć pożyczka współdzielona i mutowalna
* Nie mogą istnieć jednocześnie dwie pożyczki mutowalane

---

## Ale... Po co to?

----

## Inwalidacja iteratorów

```cpp
// iter/main.cpp
std::function<void ()> printer(const std::vector<int> & data);
void magic(std::vector<int> & data);

int main() {
    auto data = std::vector<int> { 871 };
    auto print = printer(data);
    magic(data);
    print();
}
```

---

## Inwalidacja iteratorów

```cpp
// iter/main.cpp
std::function<void ()> printer(const std::vector<int> & data) {
    auto beg = data.begin();
    auto end = data.end();

    return [beg, end]() {
        for(auto it = beg; it != end; ++it) {
            std::cout << "Item: " << *it << '\n';
        }
    };
}
```

---

## Inwalidacja iteratorów

```cpp
// iter/main.cpp
void magic(std::vector<int> & data) {
    int last = data.back();
    while(last != 1) {
        if(last % 2 == 0) {
            data.push_back(last / 2);
        } else {
            data.push_back(last * 3 + 1);
        }
        last = data.back();
    }
}
```

---

## Inwalidacja iteratorów - Rust

```rust
// iter/main.rs
fn printer(data: &[i32]) -> impl Fn() -> () + '_;
fn magic(data: &mut Vec<i32>);

fn main() {
    let mut data = vec![871];
    let print = printer(&data);
    magic(&mut data);
    print();
}
```

---

## Inwalidacja iteratorów

```bash
error[E0502]: cannot borrow `data` as mutable because it
              is also borrowed as immutable
  --> main.rs:26:11
   |
25 |     let print = printer(&data);
   |                         ----- immutable borrow occurs here
26 |     magic(&mut data);
   |           ^^^^^^^^^ mutable borrow occurs here
27 |     print();
   |     ----- immutable borrow later used here

error: aborting due to previous error

For more information about this error, try
`rustc --explain E0502`.
```

----

## Jak?

### Lifetimy

---

## Zasięg zmiennej

```cpp
// dangling/main.cpp
int & dangling(int arg) {
    return arg;
    // koniec zasięgu `arg`
}

int main() {
    int answer = 42;
    auto ref = dangling(answer);
    std::cout << ref << '\n';
}
```

---

## Zasięg zmiennej

```bash
main.cpp: In function ‘int& dangling(int)’:
main.cpp:4:12: warning: reference to local variable ‘arg’
                        returned [-Wreturn-local-addr]
    4 |     return arg;
      |            ^~~
main.cpp:3:20: note: declared here
    3 | int & dangling(int arg) {
      |                ~~~~^~~
./dan_cpp
make: *** [Makefile:5: run-cpp] Naruszenie ochrony pamięci
      (zrzut pamięci)
```

---

## Zasięg zmiennej - Rust

```rust
// dangling/main.rs
fn dangling(arg: i32) -> &i32 {
    &arg
    // koniec zasięgu `arg`
}

fn main() {
    let answer = 42;
    let borrow = dangling(answer);
    println!("Answer: {}", borrow);
}
```

---

## Zasięg zmiennej

```bash
error[E0106]: missing lifetime specifier
 --> main.rs:1:26
  |
1 | fn dangling(arg: i32) -> &i32 {
  |                          ^ help: consider giving it an
  |                            explicit bounded or
  |                            'static lifetime: `&'static`
  |
  = help: ...

error: aborting due to previous error

For more information about this error, try
`rustc --explain E0106`.
```

---

## Zasięg zmiennej

```bash
error[E0515]: cannot return reference to function
              parameter `arg`
 --> main1.rs:2:5
  |
2 |     &arg
  |     ^^^^ returns a reference to data owned by the
  |          current function

error: aborting due to previous error

For more information about this error, try
`rustc --explain E0515`.
```

---

## Lifetime

```rust
// dangling/main2.rs
fn borrow<'a>(arg: &'a i32) -> &'a i32 {
    &arg
}

fn main() {
    let answer = 42;
    let borrow = borrow(&answer);
    println!("Answer: {}", borrow);
}
```

---

## Lifetime

```rust
fn borrow<'a>(
//        ^^ Dla dowolnego miejsca w aplikacji
    arg: &'a i32
//        ^^ Dla którego pożyczka `arg` jest prawidłowa
) -> &'a i32 {
//   ^^ Zwrócona wartość musi być prawidłowa
// Co kompilator musi mieć możliwość udowodnić kompilując
// funkcję `borrow`
    arg
}
```

---

## Lifetime

```rust
fn main() {
    // Walidując poprawność pożyczek, brane są pod uwagę
    // wyłącznie sygnatury
    let answer = 42;
    // fn borrow<'a>(_: &'a i32) -> &'a i32;
    let borrow = borrow(&answer);
    // zmienna borrow może istnieć wtedy i tylko wtedy,
    // kiedy mogłaby istnieć pożyczka `&answer`
    println!("Answer: {}", borrow);
}
```

---

## Lifetime a wiszące referencje

```rust
// dangling/main3.rs
fn main() {
    let borrow = {
        let answer = 42;
        borrow(&answer)
    };
    println!("Answer: {}", borrow);
}
```

---

## Lifetime a wiszące referencje

```bash
error[E0597]: `answer` does not live long enough
 --> main3.rs:8:16
  |
6 |     let borrow = {
  |         ------ borrow later stored here
7 |         let answer = 42;
8 |         borrow(&answer)
  |                ^^^^^^^ borrowed value does not live
  |                        long enough
9 |     };
  |     - `answer` dropped here while still borrowed

For more information about this error, try
`rustc --explain E0597`.
```

----

## Inwalidacja iteratorów po raz drugi

```rust
// iter/main2.rs
fn printer<'a>(data: &'a [i32]) -> impl Fn() -> () + 'a {
    let iter = data.iter();
    move || {
        for item in iter.clone() {
            println!("Item: {}", item);
        }
    }
}
```

---

## Inwalidacja iteratorów po raz drugi

```rust
fn printer<'a>(
//         ^^ Dla dowolnego miejsca w aplikacji
  data: &'a [i32]
//       ^^ Dla którego pożyczka `data` jest prawidłowa
) ->
  impl Fn() -> ()
  + 'a
//  ^^ Zwrócona wartość musi być prawidłowa
{
  // ...
}
```

---

## Inwalidacja iteratorów po raz drugi

```rust
fn printer<'a>(data: &'a [i32]) -> impl Fn() -> () + 'a {
    // fn iter<'a>(&'a self) ->
    //   impl Iterator<Item = &'a i32> + 'a
    let iter = data.iter();
    move || {
        // `Clone::clone` zwraca taki sam typ - a więc
        // `impl Iterator<Item = &'a i32> + 'a`
        //
        // Typ `item` wynika z `Iterator::next`:
        // fn next(&mut self) -> &'a i32;
        for item in iter.clone() {
            println!("Item: {}", item);
        }
    }
}

```

----

## Wyścig pamięci

```cpp
// race/main.cpp
void next_collatz(std::vector<int> * collatz);

int main() {
    std::vector<int> data = { 12 };
    std::vector<std::thread> poll;

    for(int i = 0; i < 20; ++i) {
        auto t = std::thread(next_collatz, &data);
        poll.push_back(std::move(t));
    }

    // Poczekaj na wszystkie wątki i wypisz wyniki
}
```

---

## Wyścig pamięci

```bash
double free or corruption (fasttop)
make: *** [Makefile:5: run-cpp] Przerwane (zrzut pamięci)
```

```bash
make: *** [Makefile:5: run-cpp] Naruszenie ochrony pamięci
(zrzut pamięci)
```

---

## Mutex

```cpp
// race/main1.cpp
void next_collatz(std::vector<int> * collatz, std::mutex * mtx);

int main() {
    std::mutex mtx;
    std::vector<int> data = { 12 };
    std::vector<std::thread> poll;

    for(int i = 0; i < 20; ++i) {
        auto t = std::thread(next_collatz, &data, &mtx);
        poll.push_back(std::move(t));
    }

    // Poczekaj na wszystkie wątki i wypisz wyniki
}
```

---

## Wyścig pamięci - Rust

```rust
// race/main.rs
fn next_collatz(collatz: &mut Vec<i32>);

fn main() {
    let mut data = vec![12];
    let mut poll = vec![];

    for _ in 0..20 {
        let t = std::thread::spawn(|| next_collatz(&mut data));
        poll.push(t);
    }

    // Poczekaj na zakończenie wątków i wypisz wyniki
}
```

Note:
3 Błędy kompilacji (3 slajdy)

---

## Wyścig pamięci - Rust

```bash
error[E0499]: cannot borrow `data` as mutable more than
              once at a time
  --> main.rs:17:36
   |
17 |         let t = std::thread::spawn(|| ...);
   |                 -------------------^^-----
   |                 |                  |  |
   |                 |                  |  borrows occur due to
   |                 |                  |  use of `data` in
   |                 |                  |  closure
   |                 |                  mutable borrow starts
   |                 |                  here in previous
   |                 |                  iteration of loop
   |                 argument requires that `data` is borrowed
   |                 for `'static`
```

---

## Wyścig pamięci

```bash
error[E0373]: closure may outlive the current function, but it
              borrows `data`, which is owned by the current
              function
  --> main.rs:17:36
   |
17 |         let t = std::thread::spawn(
   |           || next_collatz(&mut data));
   |           ^^                   ---- `data` is borrowed here
   |           |
   |           may outlive borrowed value `data`
   |
```

---

## Wyścig pamięci

```bash
error[E0505]: cannot move out of `data` because it is borrowed
  --> main.rs:25:17
   |
17 |         let t = std::thread::spawn(
   |           || next_collatz(&mut data));
   |           ----------------------------
   |           |               |        |
   |           |               |        borrow occurs due to use
   |           |               |        in closure
   |           |               borrow of `data` occurs here
   |           argument requires that `data` is borrowed
   |           for `'static`
...
25 |     for next in data {
   |                 ^^^^ move out of `data` occurs here
```

---

## Spróbujmy Mutexa

```rust
// race/main2.rs
fn next_collatz(collatz: &std::sync::Mutex<Vec<i32>>);

fn main() {
    let data = std::sync::Mutex::new(vec![12]);
    let mut poll = vec![];

    for _ in 0..20 {
        let t = std::thread::spawn(|| next_collatz(&data));
        poll.push(t);
    }

    // Poczekaj na wątki i wypisz wynik
}
```

---

## Lepiej, ale wciąż...

```bash
error[E0373]: closure may outlive the current function, but it
              borrows `data`, which is owned by the
              current function
  --> main1.rs:18:36
   |
18 |         let t = std::thread::spawn(
   |           || next_collatz(&data)
   |           ^^               ---- `data` is borrowed here
   |           |
   |           may outlive borrowed value `data`
   |
```

---

## Czym jest ten `'static`?

Specjalny lifetime oznaczający "jest poprawny tak długo, jak długo żyje aplikacja"

W przypadku pożyczek, muszą to być pożyczki do stałych czasu kompilacji

W przypadku innych typów - muszą to być typy zawierające tylko pola `'static`

---

## Zliczanie referencji

```rust
// race/main2.rs
fn next_collatz(collatz: Arc<Mutex<Vec<i32>>>);

fn main() {
    let data = Arc::new(Mutex::new(vec![12]));
    let mut poll = vec![];

    for _ in 0..20 {
        let data = data.clone();
        let t = std::thread::spawn(move || next_collatz(data));
        poll.push(t);
    }

    // ...
}
```

---

## A co z zero-everhead?

```rust
// crossbeam/src/main.rs
fn next_collatz(collatz: &Mutex<Vec<i32>>);

fn main() {
    let data = Mutex::new(vec![12]);

    crossbeam::scope(|s| {
        for _ in 0..20 {
            s.spawn(|_| next_collatz(&data));
        }
    })
    .unwrap();

    // Wypisz wyniki
}
```

----

## Zliczanie referencji - a więc wycieki pamięci?

# TAK!

Rust _NIE_ chroni przed wyciekami pamięci bardziej, niż wzorzec RAII.

---

## Czy jest to problem?

### Nie aż tak wielki

---

## Cykl referencji

```rust
// leak/main.rs
fn main() {
    let a = Rc::new(A {
        data: 42,
        ptr: RefCell::new(None)
    });
    // `b::data` points to `a`
    let b = Rc::new(B {
        data: 1024,
        ptr: a.clone()
    });
    // `a::data` points to `b`
    a.ptr.replace(Some(b.clone()));
}
```

----

## Co z pożyczkami w strukturach?

```rust
// struct/main.rs
struct Circle {
    radius: &f32,
}

fn main() {
    let circle = {
        let radius = 7.0f32;
        Circle {
            radius: &radius,
        }
    };

    println!("Area: {}", circle.area());
}
```


---

## Pożyczki w strukturach

```bash
error[E0106]: missing lifetime specifier
 --> main.rs:2:13
  |
2 |     radius: &f32,
  |             ^ expected named lifetime parameter
  |
help: consider introducing a named lifetime parameter
  |
1 | struct Circle<'a> {
2 |     radius: &'a f32,
  |
```

---

## Generalizacja ponad lifetimami

```rust
// structs/main1.rs
struct Circle<'r> {
    radius: &'r f32,
}

fn main() {
    let circle = {
        let radius = 7.0f32;
        Circle {
            radius: &radius,
        }
    };

    println!("Area: {}", circle.area());
}
```

---

## Generalizacja ponad lifetimami

```bash
error[E0597]: `radius` does not live long enough
  --> main1.rs:15:21
   |
12 |     let circle = {
   |         ------ borrow later stored here
...
15 |             radius: &radius,
   |                     ^^^^^^^ borrowed value does not
   |                             live long enough
16 |         }
17 |     };
   |     - `radius` dropped here while still borrowed
```

---

## Lifetime i struktury

```rust
// structs/main2.rs
fn main() {
    let radius = 7.0f32;
    let circle = {
        Circle {
            radius: &radius,
        }
    };

    println!("Area: {}", circle.area());
}
```

---

## Lifetime i struktury

```rust
// structs/main3.rs
impl<'r> Circle<'r> {
    fn get_radius(&self) -> &'r f32 {
        self.radius
    }
}

fn main() {
    let radius = 7.0f32;
    let borrow = {
        let circle = Circle { radius: &radius };
        circle.get_radius()
    };
}
```

---

## Lifetime i struktury

```rust
// structs/main4.rs
impl<'r> Circle<'r> {
    fn get_radius<'a>(&'a self) -> &'a f32 {
        self.radius
    }
}

fn main() {
    let radius = 7.0f32;
    let borrow = {
        let circle = Circle { radius: &radius };
        circle.get_radius()
    };
}
```

---

## Lifetime i struktury

```bash
error[E0597]: `circle` does not live long enough
  --> main4.rs:18:9
   |
13 |     let borrow = {
   |         ------ borrow later stored here
...
18 |         circle.get_radius()
   |         ^^^^^^ borrowed value does not live long enough
19 |     };
   |     - `circle` dropped here while still borrowed
```

---

## Lifetimy i struktury

```rust
// structs/main5.rs
struct Slicer<'a> {
    part: &'a [i32],
}

impl<'a> Slicer<'a> {
    fn limit(&self, pred: impl Fn(i32) -> bool) -> Slicer<'a> {
        // ...
    }
}
```

---

## Lifetimy i struktury

```rust
// structs/main5.rs
fn main() {
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let slicer = Slicer { part: &data };
    println!("Slicer: {:?}", slicer);
    let slicer = slicer.limit(|x| x > 3);
    println!("Slicer: {:?}", slicer);
    let slicer = slicer.limit(|x| x % 2 == 1); println!("Slicer: {:?}", slicer);
}
```

---

## Lifetimy i struktury

```rust
// structs/main6.rs
fn main() {
    let mut data = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let slicer = Slicer { part: &data };
    println!("Slicer: {:?}", slicer);
    let slicer = slicer.limit(|x| x > 3);
    println!("Slicer: {:?}", slicer);
    data.push(24);
    let slicer = slicer.limit(|x| x % 2 == 1);
    println!("Slicer: {:?}", slicer);
}
```

---

## Lifetimy i struktury

```bash
error[E0502]: cannot borrow `data` as mutable because it is
              also borrowed as immutable
  --> main6.rs:26:5
   |
22 |     let slicer = Slicer { part: &data };
   |                                 ----- immutable borrow
   |                                       occurs here
...
26 |     data.push(24);
   |     ^^^^^^^^^^^^^ mutable borrow occurs here
27 |     let slicer = slicer.limit(|x| x % 2 == 1);
   |                  ------ immutable borrow later used here
```

----

## Lifetimy i traity

```rust
// traits/main.rs
trait Slicer<'a> {
    type Output: 'a;

    fn limit(&self, pred: impl Fn(i32) -> bool) -> Self::Output;
}

impl<'a> Slicer<'a> for &'a [i32] {
    type Output = &'a [i32];

    fn limit(&self, pred: impl Fn(i32) -> bool) -> &'a [i32] {
        // ...
    }
}
```

----

## Pytania

---

## Rust Wrocław

* www.rust-wroclaw.pl
* [fb/rustwroclaw](http://facebook.com/rustwroclaw)
* [yt/channel/UC9X86dyEwpbCnpC18qjt33Q](http://youtube.com/channel/UC9X86dyEwpbCnpC18qjt33Q)

---

# Dziękuję

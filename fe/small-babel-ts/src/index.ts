const a = 1;
let b = 2;

{
  const a = 3;
  let b = 4;
}

const c = (d: Number) => d;

class e {
  f: any;
  constructor(f: any) {
    this.f = f;
  }
  get() {
    return this.f;
  }
  set(g: any) {
    this.f = g;
  }
}

function h(i: Number = 10) {
  console.log(i);
}
h();
h(15);

const [j, k, l] = [1, 2, { m: 3 }];

const o = [1, 10, 32, 2];
console.log(Math.max(...o));

console.log([1, 2, 3, 4].includes(2));

console.log("test hello".match(/(?<=test )hello/g));

// class C {
//     private x = 10
//     getX = () => this.x;
//     setX = (newVal: number) => { this.x = newVal; }
// }

// let x = new C();
// let y = { ...{ some: "value" } }

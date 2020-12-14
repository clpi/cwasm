import load from "./build/cdom.js";

async function cdom() {
  let cdom = load().catch(console.error)
    .then(cdom => cdom.greet());
  return cdom;
}

let cd = cdom().then(c => {
  console.log(c);
  return c
})

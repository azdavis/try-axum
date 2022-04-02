"use strict";

const points = document.getElementById("points");
const newPoint = document.getElementById("new-point");
const xInput = document.getElementById("x");
const yInput = document.getElementById("y");

const numRe = /^\d+$/;

newPoint.addEventListener("submit", (e) => {
  e.preventDefault();
  const x = xInput.value;
  const y = yInput.value;
  if (!numRe.test(x)) {
    alert(`not a number: ${x}`);
    return;
  }
  if (!numRe.test(y)) {
    alert(`not a number: ${y}`);
    return;
  }
  fetch("/point", {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ x: parseInt(x), y: parseInt(y) }),
  });
});

async function main() {
  const res = await fetch("/points");
  const ps = await res.json();
  for (const { x, y } of ps) {
    const li = document.createElement("li");
    li.textContent = `(${x}, ${y})`;
    points.appendChild(li);
  }
}

main();

import React from 'react'

export const clientLoader = () => {
  return {
    name: 'asdasdas',
    age: 19,
  }
}

export default function Component() {
  const data = clientLoader();
  console.log(data);
  return <h1>AAAA</h1>
}

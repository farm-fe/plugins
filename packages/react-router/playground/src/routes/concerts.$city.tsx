import React from 'react'

export async function clientLoader() {
  return 'loader'
}

export async function clientAction() {
  return 'action'
}

export default function Component() {
  return <h1>Dynamic</h1>
}

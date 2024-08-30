import React, { useState } from 'react'
import { useLoaderData } from 'react-router-dom'

export async function clientLoader() {
  return 'Vite + React'
}

export default function Component() {
  const [count, setCount] = useState(0)
  const data = useLoaderData() as string

  return (
    <>
      <div className="font-sans">
      </div>
      <h1>{ data }</h1>
      <div className="card">
        <button onClick={() => setCount(count => count + 1)}>
          count is
          {' '}
          {count}
          {' '}
          <i className="tabler-123" />
        </button>
        <p>
          Edit
          {' '}
          <code>src/App.tsx</code>
          {' '}
          and save to test HMR
        </p>
      </div>
      <p className="read-the-docs">
        Click on the Vite and React logos to learn more
      </p>
    </>
  )
}

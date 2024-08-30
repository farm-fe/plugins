// add context <Outlet context={toast}

import React from 'react'
import { Outlet } from 'react-router-dom'

export default function Component() {
  return (
    <>
      <h1>hidden auth layout</h1>
      <Outlet />
    </>
  )
}

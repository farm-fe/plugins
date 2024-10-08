import{useState}from"react";import"./main.css";import FarmLogo from"./assets/logo.png";import reactLogo from"./assets/react.svg";import{ComponentF}from"./components/ComponentE.tsx";import{Button as AntButton}from"antd";import{ComponentX}from"./components/ComponentD.tsx";import{ComponentE}from"./components/ComponentE.tsx";import{ComponentC}from"./components/ComponentC.tsx";import{ComponentB}from"./components/ComponentB.tsx";import{Space as AntSpace}from"antd";import ComponentD from"./components/ComponentD.tsx";import ComponentA from"./components/ComponentA.tsx";export function Main(){const[count,setCount]=useState(0);console.log("rendering Main component");return(<>
      <div>
        <a href="https://farmfe.org/"target="_blank">
          <img src={FarmLogo}className="logo"alt="Farm logo"/>
        </a>
        <a href="https://react.dev"target="_blank">
          <img src={reactLogo}className="logo react"alt="React logo"/>
        </a>
      </div>
      <h1>Farm + React</h1>
      count is {count}
      <div className="card">
        <ComponentA></ComponentA>
        <ComponentB></ComponentB>
        <ComponentC></ComponentC>
        <ComponentD></ComponentD>
        <ComponentE></ComponentE>
        <ComponentF></ComponentF>
        <ComponentX></ComponentX>
        <AntSpace>
          <ArcoButton type="primary"onClick={()=>setCount(count=>count+1)}>arco button</ArcoButton>
          <AntButton type="primary"onClick={()=>setCount(count=>count+1)}>antd button</AntButton>
          <button onClick={()=>setCount(count=>count+1)}>
            button
          </button>
        </AntSpace>
        <p>
          Edit <code>src/main.tsx</code> and save to test HMR
        </p>
      </div>
      <p className="read-the-docs">
        Click on the Farm and React logos to learn more
      </p>
    </>);}

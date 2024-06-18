import React,{useState}from"react";import"./main.css";import"@arco-design/web-react/dist/css/arco.css";
import reactLogo from"./assets/react.svg";
import FarmLogo from"./assets/logo.png";
import CompC from"/Users/bytedance/Documents/open/farm-fe/plugins/packages/farm-plugin-react-components/playground/src/components/CompC.tsx";
import{CompA}from"/Users/bytedance/Documents/open/farm-fe/plugins/packages/farm-plugin-react-components/playground/src/components/CompA.tsx";
import{Empty as AntEmpty}from"antd";import{Button as AntButton}from"antd";
import{Button as AButton}from"@arco-design/web-react/es/Button/style/index";
import"@arco-design/web-react";
import{Input as AntInput}from"antd";
export function Main(){const[count,setCount]=useState(0);console.log("rendering Main component");return(<>

    <div>

      <a href="https://farmfe.org/"target="_blank">

        <img src={FarmLogo}className="logo"alt="Farm logo"/>

      </a>

      <a href="https://react.dev"target="_blank">

        <img src={reactLogo}className="logo react"alt="React logo"/>

      </a>

    </div>

    <h1>Farm + React</h1>

    <CompA title=""></CompA>

    <CompC></CompC>

    <AntButton onClick={()=>setCount(count=>count+1)}>AntButton</AntButton>

    <AntEmpty/>

    <AntInput></AntInput>

    <AButton>asdasdsa</AButton>

    <div className="card">

      <button onClick={()=>setCount(count=>count+1)}>

        count is {count}

      </button>

      <p>

        Edit <code>src/main.tsx</code> and save to test HMR

      </p>

    </div>

    <p className="read-the-docs">

      Click on the Farm and React logos to learn more

    </p>

  </>);}

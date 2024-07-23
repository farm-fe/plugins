import './main.css';
// @ts-ignore
import { a } from 'virtualModule';

export function Main() {
  return (
    <>
      <div className='virtual-module'>{a}</div>
    </>
  );
}

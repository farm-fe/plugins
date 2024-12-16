import React, { useCallback, useEffect, useState } from 'react';

export default function App() {
    const [count, setCount] = useState(1);

    useEffect(() => {
        console.log(count);
    }, [count]);

    const inc = useCallback(() => setCount(v => v + 1), []);

    return (
        <div>
            <h1>hello world {count}</h1>
            <button onClick={() => inc()}>inc</button>
        </div>
    );
}

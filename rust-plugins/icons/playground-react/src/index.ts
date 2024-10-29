const task = [
  A1,


  B1,
  B2
]

close(task.find(t => t.type === "b"))

function append(...ts) {
  const A = task.filter()
  const B = task.filter()

  ts.forEach(t => {
    if (task == 3) {
      if (t.type == "b") {
        if (A.length === 3) {
          // todo
        } else {
          task.push(t)
          // settimout
        }
      }

      if (t.type === "a") {
        close(B.shift())
      }
    } else {
      task.push(t)
      if (t.type == "b") {
        // settimout
      }
    }
  })
}




const [
  taskIds,
  onChanage
] = useNotiy()

onChanage(({
  state,
  taskId
}) => {
  if (state === open) {
    console.log(taskId);
  }
})


const useNotiy = ()=>{
  const fns = []
  const close = (state)=>{
    fns.forEach(fn=>{
      fn(state)
    })
  }

  /// close

  return {
    onChanage(fn){
      fns.push(fn)
    }

  }
}

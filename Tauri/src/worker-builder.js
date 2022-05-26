

export default class WorkerBuilder extends Worker {
  constructor(worker) {
    const code = worker.toString();
    const blob = new Blob([`(${code})()`], {type : 'module'});
    console.log(blob);
    return new Worker(URL.createObjectURL(blob));
    //return new Worker(new URL('./deep-thought.js', import.meta.url));
  }
}
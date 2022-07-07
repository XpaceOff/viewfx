export default class WorkerBuilder extends Worker {
    constructor(worker) {
      const code = worker.toString();
      const blob = new Blob([`(${code})()`], {type : 'module'});
      console.log(blob);
      return new Worker(URL.createObjectURL(blob));
    }
  }
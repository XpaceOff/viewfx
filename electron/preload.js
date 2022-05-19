

window.addEventListener('DOMContentLoaded', () => {
    const replaceText = (selector, text) => {
        console.log(selector);
        console.log("text: ", text);
        const element = document.getElementById(selector)
        if (element) element.innerText = text
    }
  
    for (const dependency of ['chrome', 'node', 'electron']) {
        replaceText(`${dependency}-version`, process.versions[dependency])
    }
})


// import Modules
const { app, BrowserWindow } = require('electron')
const path = require('path')

// Create the main window
const createWindow = () => {
    const win = new BrowserWindow({
      width: 900,
      height: 600,
      webPreferences: {
        preload: path.join(__dirname, 'preload.js')
      }
    })
  
    win.loadFile('index.html')
}

// This method will be called when Electron has finished
// initialization and is ready to create browser windows.
// Some APIs can only be used after this event occurs.
app.whenReady().then(() => {
    createWindow()

    app.on('activate', () => {
        // On macOS it's common to re-create a window in the app when the
        // dock icon is clicked and there are no other windows open.
        if (BrowserWindow.getAllWindows().length === 0) createWindow()
    })
})

// Quit when all windows are closed, except on macOS
app.on('window-all-closed', () => {
    if (process.platform !== 'darwin') app.quit()
})


// --- GPU related

app.on('gpu-info-update', () => {
    console.log('GPU Information has been Updated');

    //console.dir(app.getGPUFeatureStatus());
});

app.on('gpu-process-crashed', (event, killed) => {
    console.log('GPU Process has crashed');
    console.log(event);
    console.log('Whether GPU Process was killed - ', killed);
});
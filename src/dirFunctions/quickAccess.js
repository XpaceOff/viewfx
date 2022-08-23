import { path } from "@tauri-apps/api"

export async function getQuickAccessDirs(){
    let rList = [];
    
    try {
        let homeDir = await path.homeDir();
        let desktopDir = await path.desktopDir();
        let documentDir = await path.documentDir();
        let downloadDir = await path.downloadDir();

        if (homeDir != "") rList.push(["Home", homeDir]);
        if (desktopDir != "") rList.push(["Desktop", desktopDir]);
        if (documentDir != "") rList.push(["Documents", documentDir]);
        if (downloadDir != "") rList.push(["Downloads", downloadDir]);

    } catch (error) {
        console.log(error);
    }

    return(rList);
}

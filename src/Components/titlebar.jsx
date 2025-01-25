import styles from './../styles/titlebar.module.css';
import {invoke} from "@tauri-apps/api/core";

export default function Titlebar() {

    const onExit = async () => {
        await invoke("on_exit")
    }
    const onMax = async () => {
        await invoke("on_max")

    }
    const onMin = async () => {
        await invoke("on_min")
    }

    return <div className={styles.titlebar}>
        <div className={styles.logo}>
            <img src="./icon.png" alt="logo"/>
            <h4>Mehrnux</h4>
        </div>
        <div className={styles.controlbars}>
            <button onClick={onMin}></button>
            <button onClick={onMax}></button>
            <button onClick={onExit}></button>
        </div>
    </div>
}
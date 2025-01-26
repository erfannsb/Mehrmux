import styles from "./../styles/dino.module.scss"

export default function Dino() {
    return (
        <div className={styles.dino}>
            <div className={styles.eye}></div>
            <div className={styles.mouth}></div>
            <div className={styles.ground}></div>
            <div className={styles.comets}></div>
        </div>
    )
}
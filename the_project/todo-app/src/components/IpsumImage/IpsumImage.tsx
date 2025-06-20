import Image from 'next/image'
import styles from './IpsumImage.module.scss'

export default function IpsumImage() {
  return (
    <div className={styles.imageContainer}>
      <Image src="/image.jpg" alt="Picsum Image" width={200} height={300} />
    </div>
  )
}

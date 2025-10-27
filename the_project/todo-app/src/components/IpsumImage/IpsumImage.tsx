import Image from 'next/image'
import styles from './IpsumImage.module.scss'
import image from '@/../public/image.jpg'

export default function IpsumImage() {
  return (
    <div className={styles.imageContainer}>
      <Image src={image} alt="Picsum Image" width={200} height={300} />
    </div>
  )
}

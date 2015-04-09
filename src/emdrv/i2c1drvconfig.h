/***************************************************************************//**
 * @file i2c1drvconfig.h
 * @brief I2C1DRV configuration file.
 * @version
 *******************************************************************************
 * @section License
 * <b>(C) Copyright 2014 Silicon Labs, http://www.silabs.com</b>
 *******************************************************************************
 *
 * This file is licensed under the Silabs License Agreement. See the file
 * "Silabs_License_Agreement.txt" for details. Before using this software for
 * any purpose, you must agree to the terms of that agreement.
 *
 ******************************************************************************/

#ifndef __SILICON_LABS_I2C1DRV_CONFIG_H__
#define __SILICON_LABS_I2C1DRV_CONFIG_H__

/***************************************************************************//**
 * @addtogroup EM_Drivers
 * @{
 ******************************************************************************/

 /***************************************************************************//**
 * @addtogroup I2CDRV
 * @{
 ******************************************************************************/
/* Use location 1: SDA - Pin B11, SCL - Pin B12 */
#define I2C1DRV_SCL_PORT gpioPortC
#define I2C1DRV_SCL_PIN  5
#define I2C1DRV_SDA_PORT gpioPortC
#define I2C1DRV_SDA_PIN  4
#define I2C1DRV_PORT_LOCATION 0
#define I2C1DRV_TRANSFER_TIMEOUT 300000

/** @} (end addtogroup I2CDRV) */
/** @} (end addtogroup EM_Drivers) */

#endif /* __SILICON_LABS_I2CDRV_CONFIG_H__ */

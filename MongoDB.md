# Setup a MongoDB Account

MongoDB is a versatile and widely-used NoSQL database ideal for managing data in dynamic, scalable applications. Follow these steps to set up MongoDB for your project:

## Create a MongoDB Account

1. **Sign Up for MongoDB Atlas**
   Go to [MongoDB Atlas](https://account.mongodb.com/account/login) and create a new account if you don't have one. You'll need to provide a few basic details.

1. **Deploy a New Cluster**
   - After logging in, go to your MongoDB Atlas dashboard and click **Create** to start setting up your first cluster.

1. **Choose the Free Tier**
   For initial testing and development, select the free tier option. This gives you a cluster at no cost and is great for getting started.

1. **Set Up Database User Credentials**
   - Create a username and password for accessing your database.
   - Click "Create User" to save the credentials. Remember to store these credentials securely since you'll need them to connect from your app.

1. **Configure IP Access List**
   - For development, set the IP access list to allow connections from any IP by adding `0.0.0.0/0`.
   - *Note*: In production, restrict access to specific IPs for enhanced security.

## Connect to the MongoDB Cluster

1. **Navigate to the Database Section**
   In the MongoDB Atlas dashboard, locate the left sidebar and click on "Database."

1. **Click "Connect" on Your Cluster**
   - Select the cluster you just created and click "Connect". MongoDB will display connection options for different environments.

1. **Copy the Connection String (Cluster URL)**
   - Choose the connection option appropriate for your environment (e.g., "Connect your application") and copy the connection string (usually starts with `mongodb+srv://`).

## Configure MongoDB Credentials in Your Project

Now that your MongoDB cluster is ready, you'll configure your project to connect to it.

1. **Set Environment Variables**
   - Open your project's `.env` file or set one if it doesn't exist.

1. **Add MongoDB Credentials**
   - Insert the following details into the `.env` file, substituting the placeholder values with the username, password, and connection URL you copied earlier:
     ```plaintext
     # MongoDB Configuration
     MONGODB_USR=
     MONGODB_PWD=
     MONGODB_CLSTR=your-cluster.mongodb.net
     MONGODB_DB_NAME=aibooks
     ```
   - Replace `<MONGODB_USR>`, `<MONGODB_PWD>`, and `<MONGODB_CLSTR>` with your specific information. `MONGODB_DB_NAME` can be any database name you'd like; for this guide, we'll use "aibook".

1. **Save Your Configuration**
   Save the `.env` file after entering all details.

> **Security Note**
> Keeping your MongoDB credentials secure is crucial, especially in a production environment. Consider using secure storage solutions for environment variables, such as secrets management tools.


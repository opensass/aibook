# Set Up Upstash Redis Env Vars

1. **Create a Redis Database**

   - Go to the [Upstash Console](https://console.upstash.com/auth/sign-in).
   - **Create a Database**:

     - Click **Create a Redis Database**.

     - Enter Database details.

1. **Retrieve Your Endpoint URL**

   - After creating a database, note the **Endpoint URL**.

1. **Set the Environment Variable**
   Add the following to your `.env` file:

   ```env
   REDIS_URL=redis://default:....upstash.io:6379
   ```

Your environment variables are set and ready for Redis integration.

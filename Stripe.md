# Set Up Stripe Env Vars

1. **Create Products and Pricing in Stripe**

   - Go to the [Stripe Dashboard](https://dashboard.stripe.com/).
   - **Create a Product**:

     - Navigate to **Product Catalog > Create Product**.

     ![add product](https://github.com/user-attachments/assets/46a2ca96-4769-48a4-b7a4-5188e030e9e5)

     - Enter product details (e.g., "Monthly Plan", "Yearly Plan").

     ![Monthly Plan](https://github.com/user-attachments/assets/3fba067e-143c-455c-9d44-96b042a3560f)
     ![Yearly Plan](https://github.com/user-attachments/assets/eac0da1e-a14b-4f36-98f2-c4107f4cfc53)

1. **Retrieve Your Product Price IDs**

   - After creating products, note the **Price IDs** (e.g., `price_1...`) from the pricing section.

     - **STRIPE_PRICE_ONE** = `price_1AbCDeFgHiJkLmNOpQrStUv` (Monthly Plan).
     - **STRIPE_PRICE_TWO** = `price_1XyZaBcDeFgHiJkLmNoQrStUv` (Yearly Plan).

     ![price id](https://github.com/user-attachments/assets/4ac40893-40d3-4cb4-83a6-45dd6e130a4a)
     ![price id](https://github.com/user-attachments/assets/709ea1f6-5543-428d-ad9a-d4164d4f0762)
     ![price id](https://github.com/user-attachments/assets/fe5d558d-79cc-48cd-9d52-7de8e48bfe35)
     ![price id](https://github.com/user-attachments/assets/9a6f7cfa-e609-460e-9501-043d1c8b2880)

1. **Set the server side Environment Variables**
   Add the following to your `.env` file:

   ```env
   STRIPE_SECRET_KEY=sk_...
   WEBSITE_URL=http://0.0.0.0:3000
   ```

1. **Set the client side Environment Variables**
   Set the following price values in the terminal:

   ```env
   export STRIPE_PRICE_ONE=price_...
   export STRIPE_PRICE_TWO=price_...
   ```

Your environment variables are set and ready for Stripe integration.

use crate::config::{Client, Response};
use crate::ids::CustomerId;
use crate::resources::{
    CheckoutSession, CheckoutSessionLocale, CheckoutSessionMode, CheckoutSessionSubmitType,
    Currency,
};
use serde_derive::{Deserialize, Serialize};

/// The parameters for `CheckoutSession::create`
///
/// For more details see [https://stripe.com/docs/api/payment_methods/attach](https://stripe.com/docs/api/payment_methods/attach).
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateCheckoutSession<'a> {
    /// The URL the customer will be directed to if they decide to cancel payment and return to your website.
    pub cancel_url: &'a str,

    /// A list of the types of payment methods (e.g. card) this Checkout Session is allowed to accept. The only supported values today are `card` and `ideal`.
    pub payment_method_types: Vec<&'a str>,

    /// The URL the customer will be directed to after the payment or subscription creation is successful.
    pub success_url: &'a str,

    /// A unique string to reference the Checkout Session.
    ///
    /// This can be a customer ID, a cart ID, or similar, and can be used to reconcile the session with your internal systems.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_reference_id: Option<&'a str>,

    /// The ID of the customer for this session.
    ///
    /// A new customer will be created unless an existing customer was provided in when the session was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<CustomerId>,

    /// If provided, this value will be used when the Customer object is created.
    /// If not provided, customers will be asked to enter their email address.
    /// Use this parameter to prefill customer data if you already have an email
    /// on file.
    ///
    /// To access information about the customer once a session is complete, use the `customer` field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_email: Option<&'a str>,

    /// The value (`auto` or `required`) for whether Checkout should collect the customer's billing address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_address_collection: Option<&'a str>,

    /// The line items, plans, or SKUs purchased by the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_items: Option<Vec<CheckoutSessionLineItem<'a>>>,

    /// The IETF language tag of the locale Checkout is displayed in.
    ///
    /// If blank or `auto`, the browser's locale is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<CheckoutSessionLocale>,

    /// The mode of the Checkout Session, one of `payment`, `setup`, or `subscription`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<CheckoutSessionMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discounts: Option<Vec<CheckoutDiscount>>,

    // A subset of parameters to be passed to PaymentIntent creation for Checkout Sessions in payment mode
    // TODO: payment_intent_data

    // A subset of parameters to be passed to SetupIntent creation for Checkout Sessions in setup mode.
    // TODO: setup_intent_data
    /// Describes the type of transaction being performed by Checkout in order
    /// to customize relevant text on the page, such as the submit button.
    /// `submit_type` can only be specified on Checkout Sessions using line
    /// items or a SKU, but not Checkout Sessions for subscriptions.
    ///
    /// Supported values are `auto`, `book`, `donate`, or `pay`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit_type: Option<CheckoutSessionSubmitType>,
    // A subset of parameters to be passed to subscription creation for Checkout Sessions in subscription mode.
    // TODO: subscription_data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_data: Option<SubscriptionData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_tax: Option<CheckoutAutomaticTax>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CheckoutAutomaticTax {
    pub enabled: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CheckoutDiscount {
    pub coupon: Option<crate::CouponId>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SubscriptionData {
    // A subset of parameters to be passed to subscription creation for Checkout Sessions in subscription mode.
    // Hide child parameters
    // application_fee_percent optional
    // A non-negative decimal between 0 and 100, with at most two decimal places. This represents the percentage of the subscription invoice subtotal that will be transferred to the application owner’s Stripe account. To use an application fee percent, the request must be made on behalf of another account, using the Stripe-Account header or an OAuth key. For more information, see the application fees documentation.
    // default_tax_rates optional
    // A list of items, each with an attached plan, that the customer is subscribing to. Prefer using line_items.
    // Show child parameters
    // metadata optional dictionary
    // Set of key-value pairs that you can attach to an object. This can be useful for storing additional information about the object in a structured format. Individual keys can be unset by posting an empty value to them. All keys can be unset by posting an empty value to metadata.
    // transfer_data optional dictionary
    // If specified, the funds from the subscription’s invoices will be transferred to the destination and the ID of the resulting transfers will be found on the resulting charges.
    // trial_end optional
    // Unix timestamp representing the end of the trial period the customer will get before being charged for the first time. Has to be at least 48 hours in the future.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_period_days: Option<i32>,
    // Integer representing the number of trial period days before the customer is charged for the first time. Has to be at least 1.
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CheckoutSessionLineItem<'a> {
    /// The amount to be collected per unit of the line item.
    pub amount: Option<i64>,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: Option<Currency>,

    /// The name for the line item.
    pub name: Option<&'a str>,

    /// The quantity of the line item being purchased.
    pub quantity: u64,

    /// The description for the line item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,

    /// A list of images representing this line item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<String>>,
    // TODO: remaining optional fields
    pub price: Option<crate::PriceId>,
}

impl CheckoutSession {
    /// Attach a payment method to a customer
    ///
    /// For more details see [https://stripe.com/docs/api/payment_methods/attach](https://stripe.com/docs/api/payment_methods/attach).
    pub fn create(client: &Client, params: CreateCheckoutSession) -> Response<CheckoutSession> {
        client.post_form("/checkout/sessions", params)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BillingPortal {
    pub url: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateBillingPortal {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<CustomerId>,
}

impl BillingPortal {
    /// Attach a payment method to a customer
    ///
    /// For more details see [https://stripe.com/docs/api/payment_methods/attach](https://stripe.com/docs/api/payment_methods/attach).
    pub fn create(client: &Client, params: CreateBillingPortal) -> Response<BillingPortal> {
        client.post_form("/billing_portal/sessions", params)
    }
}

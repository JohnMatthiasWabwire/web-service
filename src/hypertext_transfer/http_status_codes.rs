#![allow(dead_code)]

use std::{primitive::str, vec::Vec};

// Hypertext Transfer Protocol Status Code Definition
pub type HttpStatusCode = &'static str;

// Hypertext Transfer Protocol Status Codes
pub const HTTP_ONE_HUNDRED: HttpStatusCode = "100"
pub const HTTP_ONE_HUNDRED_ONE: HttpStatusCode = "101"
pub const HTTP_ONE_HUNDRED_TWO: HttpStatusCode = "102"
pub const HTTP_ONE_HUNDRED_THREE: HttpStatusCode = "103"
pub const HTTP_TWO_HUNDRED: HttpStatusCode = "200"
pub const HTTP_TWO_HUNDRED_ONE: HttpStatusCode = "201"
pub const HTTP_TWO_HUNDRED_TWO: HttpStatusCode = "202"
pub const HTTP_TWO_HUNDRED_THREE: HttpStatusCode = "203"
pub const HTTP_TWO_HUNDRED_FOUR: HttpStatusCode = "204"
pub const HTTP_TWO_HUNDRED_FIVE: HttpStatusCode = "205"
pub const HTTP_TWO_HUNDRED_SIX: HttpStatusCode = "206"
pub const HTTP_TWO_HUNDRED_SEVEN: HttpStatusCode = "207"
pub const HTTP_TWO_HUNDRED_EIGHT: HttpStatusCode = "208"
pub const HTTP_TWO_HUNDRED_TWENTY_SIX: HttpStatusCode = "226"
pub const HTTP_THREE_HUNDRED: HttpStatusCode = "300"
pub const HTTP_THREE_HUNDRED_ONE: HttpStatusCode = "301"
pub const HTTP_THREE_HUNDRED_TWO: HttpStatusCode = "302"
pub const HTTP_THREE_HUNDRED_THREE: HttpStatusCode = "303"
pub const HTTP_THREE_HUNDRED_FOUR: HttpStatusCode = "304"
pub const HTTP_THREE_HUNDRED_SEVEN: HttpStatusCode = "307"
pub const HTTP_THREE_HUNDRED_EIGHT: HttpStatusCode = "308"
pub const HTTP_FOUR_HUNDRED: HttpStatusCode = "400"
pub const HTTP_FOUR_HUNDRED_ONE: HttpStatusCode = "401"
pub const HTTP_FOUR_HUNDRED_TWO: HttpStatusCode = "402"
pub const HTTP_FOUR_HUNDRED_THREE: HttpStatusCode = "403"
pub const HTTP_FOUR_HUNDRED_FOUR: HttpStatusCode = "404"
pub const HTTP_FOUR_HUNDRED_FIVE: HttpStatusCode = "405"
pub const HTTP_FOUR_HUNDRED_SIX: HttpStatusCode = "406"
pub const HTTP_FOUR_HUNDRED_SEVEN: HttpStatusCode = "407"
pub const HTTP_FOUR_HUNDRED_EIGHT: HttpStatusCode = "408"
pub const HTTP_FOUR_HUNDRED_NINE: HttpStatusCode = "409"
pub const HTTP_FOUR_HUNDRED_TEN: HttpStatusCode = "410"
pub const HTTP_FOUR_HUNDRED_ELEVEN: HttpStatusCode = "411"
pub const HTTP_FOUR_HUNDRED_TWELVE: HttpStatusCode = "412"
pub const HTTP_FOUR_HUNDRED_THIRTEEN: HttpStatusCode = "413"
pub const HTTP_FOUR_HUNDRED_FOURTEEN: HttpStatusCode = "414"
pub const HTTP_FOUR_HUNDRED_FIFTEEN: HttpStatusCode = "415"
pub const HTTP_FOUR_HUNDRED_SIXTEEN: HttpStatusCode = "416"
pub const HTTP_FOUR_HUNDRED_SEVENTEEN: HttpStatusCode = "417"
pub const HTTP_FOUR_HUNDRED_EIGHTEEN: HttpStatusCode = "418"
pub const HTTP_FOUR_HUNDRED_TWENTY_ONE: HttpStatusCode = "421"
pub const HTTP_FOUR_HUNDRED_TWENTY_TWO: HttpStatusCode = "422"
pub const HTTP_FOUR_HUNDRED_TWENTY_THREE: HttpStatusCode = "423"
pub const HTTP_FOUR_HUNDRED_TWENTY_FOUR: HttpStatusCode = "424"
pub const HTTP_FOUR_HUNDRED_TWENTY_FIVE: HttpStatusCode = "425"
pub const HTTP_FOUR_HUNDRED_TWENTY_SIX: HttpStatusCode = "426"
pub const HTTP_FOUR_HUNDRED_TWENTY_EIGHT: HttpStatusCode = "428"
pub const HTTP_FOUR_HUNDRED_TWENTY_NINE: HttpStatusCode = "429"
pub const HTTP_FOUR_HUNDRED_THIRTY_ONE: HttpStatusCode = "431"
pub const HTTP_FOUR_HUNDRED_FIFTEY_ONE: HttpStatusCode = "451"
pub const HTTP_FIVE_HUNDRED: HttpStatusCode = "500"
pub const HTTP_FIVE_HUNDRED_ONE: HttpStatusCode = "501"
pub const HTTP_FIVE_HUNDRED_TWO: HttpStatusCode = "502"
pub const HTTP_FIVE_HUNDRED_THREE: HttpStatusCode = "503"
pub const HTTP_FIVE_HUNDRED_FOUR: HttpStatusCode = "504"
pub const HTTP_FIVE_HUNDRED_FIVE: HttpStatusCode = "505"
pub const HTTP_FIVE_HUNDRED_SIX: HttpStatusCode = "506"
pub const HTTP_FIVE_HUNDRED_SEVEN: HttpStatusCode = "507"
pub const HTTP_FIVE_HUNDRED_EIGHT: HttpStatusCode = "508"
pub const HTTP_FIVE_HUNDRED_TEN: HttpStatusCode = "510"
pub const HTTP_FIVE_HUNDRED_ELEVEN: HttpStatusCode = "511"

// Hypertext Transfer Protocol Status Code Vector
pub fn status_codes_vector() -> Vec<HttpStatusCode> {
    let http_status_codes: Vec<HttpStatusCode> = Vec::from([
        HTTP_ONE_HUNDRED,
        HTTP_ONE_HUNDRED_ONE,
        HTTP_ONE_HUNDRED_TWO,
        HTTP_ONE_HUNDRED_THREE,
        HTTP_TWO_HUNDRED,
        HTTP_TWO_HUNDRED_ONE,
        HTTP_TWO_HUNDRED_TWO,
        HTTP_TWO_HUNDRED_THREE,
        HTTP_TWO_HUNDRED_FOUR,
        HTTP_TWO_HUNDRED_FIVE,
        HTTP_TWO_HUNDRED_SIX,
        HTTP_TWO_HUNDRED_SEVEN,
        HTTP_TWO_HUNDRED_EIGHT,
        HTTP_TWO_HUNDRED_TWENTY_SIX,
        HTTP_THREE_HUNDRED,
        HTTP_THREE_HUNDRED_ONE,
        HTTP_THREE_HUNDRED_TWO,
        HTTP_THREE_HUNDRED_THREE,
        HTTP_THREE_HUNDRED_FOUR,
        HTTP_THREE_HUNDRED_SEVEN,
        HTTP_THREE_HUNDRED_EIGHT,
        HTTP_FOUR_HUNDRED,
        HTTP_FOUR_HUNDRED_ONE,
        HTTP_FOUR_HUNDRED_TWO,
        HTTP_FOUR_HUNDRED_THREE,
        HTTP_FOUR_HUNDRED_FOUR,
        HTTP_FOUR_HUNDRED_FIVE,
        HTTP_FOUR_HUNDRED_SIX,
        HTTP_FOUR_HUNDRED_SEVEN,
        HTTP_FOUR_HUNDRED_EIGHT,
        HTTP_FOUR_HUNDRED_NINE,
        HTTP_FOUR_HUNDRED_TEN,
        HTTP_FOUR_HUNDRED_ELEVEN,
        HTTP_FOUR_HUNDRED_TWELVE,
        HTTP_FOUR_HUNDRED_THIRTEEN,
        HTTP_FOUR_HUNDRED_FOURTEEN,
        HTTP_FOUR_HUNDRED_FIFTEEN,
        HTTP_FOUR_HUNDRED_SIXTEEN,
        HTTP_FOUR_HUNDRED_SEVENTEEN,
        HTTP_FOUR_HUNDRED_EIGHTEEN,
        HTTP_FOUR_HUNDRED_TWENTY_ONE,
        HTTP_FOUR_HUNDRED_TWENTY_TWO,
        HTTP_FOUR_HUNDRED_TWENTY_THREE,
        HTTP_FOUR_HUNDRED_TWENTY_FOUR,
        HTTP_FOUR_HUNDRED_TWENTY_FIVE,
        HTTP_FOUR_HUNDRED_TWENTY_SIX,
        HTTP_FOUR_HUNDRED_TWENTY_EIGHT,
        HTTP_FOUR_HUNDRED_TWENTY_NINE,
        HTTP_FOUR_HUNDRED_THIRTY_ONE,
        HTTP_FOUR_HUNDRED_FIFTEY_ONE,
        HTTP_FIVE_HUNDRED,
        HTTP_FIVE_HUNDRED_ONE,
        HTTP_FIVE_HUNDRED_TWO,
        HTTP_FIVE_HUNDRED_THREE,
        HTTP_FIVE_HUNDRED_FOUR,
        HTTP_FIVE_HUNDRED_FIVE,
        HTTP_FIVE_HUNDRED_SIX,
        HTTP_FIVE_HUNDRED_SEVEN,
        HTTP_FIVE_HUNDRED_EIGHT,
        HTTP_FIVE_HUNDRED_TEN,
        HTTP_FIVE_HUNDRED_ELEVEN,
    ]);

    return http_status_codes;
}
export const validate = {
    validateEmail: function(email) {
        const regex = /^\w+@\w+\.\w+$/;

        return regex.test(email);
    },
    validatePhone: function(phone) {
        const regex = /^[+0-9]{9,15}$/;

        return regex.test(phone);
    }
}

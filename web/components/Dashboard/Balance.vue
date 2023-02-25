<template>
    <div>
        <div class="mx-auto max-w-6xl px-4 sm:px-6 lg:px-8">
            <h2 class="text-lg font-medium leading-6 text-gray-900">Balance</h2>
            <!-- Show zero balance checkbox -->
            <input type="checkbox" id="showZeroBalance" v-model="showZeroBalance" class="mt-2">
            <label for="showZeroBalance" class="ml-2">Afficher les balances à 0</label>
        </div>
        <div class="hidden sm:block">
      <div class="mx-auto max-w-6xl px-4 sm:px-6 lg:px-8">
        <div class="mt-2 flex flex-col">
          <div class="min-w-full overflow-hidden overflow-x-auto align-middle shadow sm:rounded-lg">
            <table class="min-w-full divide-y divide-gray-200">
              <thead>
              <tr>
                <th class="bg-gray-50 px-6 py-3 text-left text-sm font-semibold text-gray-900" scope="col">Symbole</th>
                <th class="bg-gray-50 px-6 py-3 text-right text-sm font-semibold text-gray-900" scope="col">Nom</th>
                <th class="bg-gray-50 px-6 py-3 text-left text-sm font-semibold text-gray-900" scope="col">Prix</th>
                <th class="bg-gray-50 px-6 py-3 text-right text-sm font-semibold text-gray-900" scope="col">Quantité</th>
                <th class="bg-gray-50 px-6 py-3 text-right text-sm font-semibold text-gray-900" scope="col">Total</th>
              </tr>
              </thead>
              <tbody class="divide-y divide-gray-200 bg-white">
              <tr v-for="price in prices" :key="price" class="bg-white">
                <td v-show="shouldDisplay(price)" class="whitespace-nowrap px-6 py-4 text-right text-sm text-gray-500">
                  <p class="text-gray-900 font-medium">{{ price.symbol }}</p>
                </td>
                <td v-show="shouldDisplay(price)" class="whitespace-nowrap px-6 py-4 text-right text-sm text-gray-500">
                  <p class="text-gray-900 font-medium">{{ price.name }}</p>
                </td>
                <td v-show="shouldDisplay(price)" class="whitespace-nowrap px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                  <p class="text-gray-900 font-medium">{{ formattedPrice(price.price) }}</p>
                </td>

                <td v-show="shouldDisplay(price)" class="whitespace-nowrap px-6 py-4 text-right text-sm text-gray-500">
                  <p>{{ findBalance(price.id) }}</p>
                </td>
                <!-- One bouton "acheter" et un bouton "vendre". Avec un background et des couleurs différentes et des bords arrondis. -->
                <td v-show="shouldDisplay(price)" class="whitespace-nowrap px-6 py-4 text-right text-sm font-medium">
                  <p class="text-gray-900 font-medium">{{ formattedPrice(price.price * findBalance(price.id)) }}</p>
                </td>
              </tr>
              </tbody>
            </table>
          </div>
        </div>
      </div>
      </div>
    </div>
</template>

<script>
export default {
    name: 'BuySales',
    data() {
      return {
        prices: [],
        modal: false,
        selectedItem: null,
        balance: 0.0,
        type: null,
        quantity: 0.0,
        balanceCrypto: null,
        showZeroBalance: false,
      }
    },
    async mounted() {
      await this.getBalanceCrypto();
      await this.getBalance();
      this.getPrices();
    },
    methods: {
      shouldDisplay(item) {
        console.log(item)
        return this.showZeroBalance || (item && this.findBalance(item.id) > 0);
      },
      findBalance(id) {
        return this.balanceCrypto && this.balanceCrypto.find(item => item.currency.id === id) && this.balanceCrypto.find(item => item.currency.id === id).quantity || 0;
      },
      getBalanceCrypto() {
        this.$axios.get('/api/crypto_balance')
          .then(response => {
            this.balanceCrypto = response.data.crypto_balance;
          })
          .catch(error => {
            console.log(error);
          });
      },
      getBalance() {
        this.$axios.get('/api/balance')
          .then(response => {
            this.balance = response.data.balance;
          })
          .catch(error => {
            console.log(error);
          });
      },
      openModal(item, type) {
        this.quantity = 0.0;
        this.selectedItem = item;
        this.modal = true;
        this.type = type;
      },
      formatAgo(date) {
        return new Date(date).toLocaleString('fr-FR', { day: 'numeric', month: 'long', year: 'numeric', hour: 'numeric', minute: 'numeric' });
      },
      formattedPrice(price) {
        return new Intl.NumberFormat('fr-FR', { style: 'currency', currency: 'USD' }).format(price);
      },
      getPrices() {
        this.$axios.get('/api/currencies_with_price')
          .then(response => {
            this.prices = response.data;
          })
          .catch(error => {
            console.log(error);
          });
      },
      doTransaction() {
        this.$axios.post('/api/transaction', {
          currency_id: this.selectedItem.id,
          transaction_type: this.type,
          quantity: this.quantity,
        })
          .then(response => {
            this.modal = false;
            this.getBalance();
            this.getBalanceCrypto();
            this.$swal({
              toast: true,
              position: 'bottom-end',
              showConfirmButton: false,
              timer: 5000,
              timerProgressBar: true,
              icon: 'success',
              title: 'Transaction effectuée\n' + (this.type === 'buy' ? '+' : '-') + this.quantity + ' ' + this.selectedItem.symbol
            });
          })
          .catch(error => {
            console.log(error);
            this.$swal({
              toast: true,
              position: 'bottom-end',
              showConfirmButton: false,
              timer: 5000,
              timerProgressBar: true,
              icon: 'error',
              title: 'Une erreur est survenue'
            });
          });
      },
    },
}
</script>
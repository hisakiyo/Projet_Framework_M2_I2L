<template>
  <div>
    <h2 class="mx-auto mt-8 max-w-6xl px-4 text-lg font-medium leading-6 text-gray-900 sm:px-6 lg:px-8">Toutes mes dernières transactions</h2>

    <div class="hidden sm:block">
      <div class="mx-auto max-w-6xl px-4 sm:px-6 lg:px-8">
        <div class="mt-2 flex flex-col">
          <div class="min-w-full overflow-hidden overflow-x-auto align-middle shadow sm:rounded-lg">
            <table class="min-w-full divide-y divide-gray-200">
              <thead>
              <tr>
                <th class="bg-gray-50 px-6 py-3 text-left text-sm font-semibold text-gray-900" scope="col">Transaction</th>
                <th class="bg-gray-50 px-6 py-3 text-left text-sm font-semibold text-gray-900" scope="col">Prix</th>
                <th class="bg-gray-50 px-6 py-3 text-right text-sm font-semibold text-gray-900" scope="col">Montant crypto</th>
                <th class="bg-gray-50 px-6 py-3 text-right text-sm font-semibold text-gray-900" scope="col">Montant en $US</th>
                <th class="bg-gray-50 px-6 py-3 text-left text-sm font-semibold text-gray-900" scope="col">Type</th>
                <th class="bg-gray-50 px-6 py-3 text-right text-sm font-semibold text-gray-900" scope="col">Date</th>
              </tr>
              </thead>
              <tbody class="divide-y divide-gray-200 bg-white">
              <tr v-for="transaction in transactions" :key="transaction.id" class="bg-white">
                <td class="whitespace-nowrap px-6 py-4 text-sm text-gray-900">
                  <p class="truncate text-gray-500 group-hover:text-gray-900">{{ transaction.id }}</p>
                </td>
                <td class="whitespace-nowrap px-6 py-4 text-right text-sm text-gray-500">
                  <span class="font-medium text-gray-900">{{ formatPrice(transaction.price) }}</span>
                </td>
                <td class="whitespace-nowrap px-6 py-4 text-right text-sm text-gray-500">
                  <span class="font-medium text-gray-900">{{ transaction.quantity }}<span class="font-medium text-gray-500"> {{ transaction.currency.symbol }}</span></span>
                </td>
                <td class="whitespace-nowrap px-6 py-4 text-right text-sm text-gray-500">
                  <span class="font-medium text-gray-900">{{ transaction.transaction_type === 'buy' ? '-' : '+' }}{{ formatPrice(Math.ceil(transaction.quantity * transaction.price * 100)/100) }}</span>
                </td>
                <td class="hidden whitespace-nowrap px-6 py-4 text-sm text-gray-500 md:block">
                  <span :class="[statusStyles[transaction.transaction_type], 'inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium capitalize']">{{ transaction.transaction_type === 'buy' ? 'Achat' : 'Vente' }}</span>
                </td>
                <td class="whitespace-nowrap px-6 py-4 text-right text-sm text-gray-500">
                  <time :datetime="transaction.datetime">{{ formatAgo(transaction.timestamp) }}</time>
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
  name: 'History',
  data() {
    return {
      transactions: null,
      statusStyles: {
        buy: 'bg-green-100 text-green-800',
        sell: 'bg-red-100 text-red-800',
      }
    }
  },
  async mounted() {
    await this.getPrices();
    this.getTransactions();
  },
  methods: {
    getTransactions() {
      this.$axios.get('/api/transactions')
        .then(response => {
          this.transactions = response.data.transactions;
        })
        .catch(error => {
          console.log(error);
        });
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
    formatAgo(date) {
      return new Date(date).toLocaleString('fr-FR', { day: 'numeric', month: 'long', year: 'numeric', hour: 'numeric', minute: 'numeric' });
    },
    formatPrice(price) {
      return new Intl.NumberFormat('fr-FR', { style: 'currency', currency: 'USD' }).format(price);
    },
  }
};
</script>
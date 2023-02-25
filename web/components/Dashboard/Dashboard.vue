<template>
  <div>
    <div class="mx-auto max-w-6xl px-4 sm:px-6 lg:px-8">
      <h2 class="text-lg font-medium leading-6 text-gray-900">Tableau de bord</h2>
      <div class="mt-2 grid grid-cols-1 gap-5 sm:grid-cols-2 lg:grid-cols-3">
        <div v-for="card in cards" :key="card.name" class="overflow-hidden rounded-lg bg-white shadow">
          <div class="p-5">
            <div class="flex items-center">
              <div class="flex-shrink-0">
              </div>
              <div class="ml-5 w-0 flex-1">
                <dl>
                  <dt class="truncate text-sm font-medium text-gray-500">{{ card.name }}</dt>
                  <dd>
                    <div class="text-lg font-medium text-gray-900">{{ card.amount }}<span class="font-medium text-gray-500"> {{ card.symbol }}</span></div>
                  </dd>
                </dl>
              </div>
            </div>
          </div>
          <div class="bg-gray-50 px-5 py-3">
            <div class="text-sm">
              <NuxtLink to="/dashboard/history/" class="font-medium text-cyan-700 hover:text-cyan-900">Voir tout</NuxtLink>
            </div>
          </div>
        </div>
        <!-- ajouter "voir plus" tout à droite avec un lien -->
        <NuxtLink to="/dashboard/balance/" class="col-span-1 flex justify-center py-12 px-4 border-2 border-dashed border-gray-200 rounded-lg hover:bg-gray-50">
          Voir plus
        </NuxtLink>
      </div>
    </div>

    <h2 class="mx-auto mt-8 max-w-6xl px-4 text-lg font-medium leading-6 text-gray-900 sm:px-6 lg:px-8">Mes 5 dernières transactions</h2>

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
                  <span class="font-medium text-gray-900">{{ transaction.transaction_type === 'buy' ? '+' : '-' }}{{ transaction.quantity }}<span class="font-medium text-gray-500"> {{ transaction.currency.symbol }}</span></span>
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

<script >
export default {
  name: 'Dashboard',
  data() {
    return {
      cards: [
        { name: 'Evaluation totale portefeuille', href: '#', icon: '', amount: 'XXXX,XX €' },
        { name: 'Solde FIAT', href: '#', icon: '', amount: 'XXXX,XX €' },
      ],
      prices: [],
      transactions: null,
       statusStyles: {
        buy: 'bg-green-100 text-green-800',
        sell: 'bg-red-100 text-red-800',
      }
    }
  },
  async mounted() {
    await this.getPrices();
    await this.getBalance();
    this.getTransactions();
    this.getCryptoBalance();
  },
  methods: {
    formatAgo(date) {
      return new Date(date).toLocaleString('fr-FR', { day: 'numeric', month: 'long', year: 'numeric', hour: 'numeric', minute: 'numeric' });
    },
    formatPrice(price) {
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
    getBalance() {
      this.$axios.get('/api/balance')
        .then(response => {
          this.cards[1].amount = this.formatPrice(response.data.balance)
        })
        .catch(error => {
          console.log(error);
        });
    },
    getTransactions() {
      this.$axios.get('/api/last_transactions')
        .then(response => {
          this.transactions = response.data.transactions;
        })
        .catch(error => {
          console.log(error);
        });
    },
    getCryptoBalance() {
      this.$axios.get('/api/crypto_balance')
        .then(response => {
          this.cards[0].amount = response.data.crypto_balance.reduce((acc, crypto) => {
            const price = this.prices.find(price => price.id === crypto.currency.id).price;
            return acc + (crypto.quantity * price);
          }, 0);
          this.cards[0].amount = this.formatPrice(this.cards[0].amount);
        })
        .catch(error => {
          console.log(error);
        });
    }
  }
};
</script>
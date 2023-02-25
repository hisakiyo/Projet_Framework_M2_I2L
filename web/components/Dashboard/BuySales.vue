<template>
    <div>
      <div v-if="modal" class="relative z-10" aria-labelledby="modal-title" role="dialog" aria-modal="true">
          <div class="fixed inset-0 bg-gray-500 bg-opacity-75 transition-opacity"></div>
          <div class="fixed inset-0 z-10 overflow-y-auto">
            <div class="flex min-h-full items-end justify-center p-4 text-center sm:items-center sm:p-0">
              <div class="relative transform overflow-hidden rounded-lg bg-white text-left shadow-xl transition-all sm:my-8 sm:w-full sm:max-w-lg">
                <div class="bg-white px-4 pt-5 pb-4 sm:p-6 sm:pb-4">
                  <div class="sm:flex sm:items-start">
                    <div class="mx-auto flex h-12 w-12 flex-shrink-0 items-center justify-center rounded-full bg-red-100 sm:mx-0 sm:h-10 sm:w-10">
                      <svg class="h-6 w-6 text-red-600" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M12 9v3.75m-9.303 3.376c-.866 1.5.217 3.374 1.948 3.374h14.71c1.73 0 2.813-1.874 1.948-3.374L13.949 3.378c-.866-1.5-3.032-1.5-3.898 0L2.697 16.126zM12 15.75h.007v.008H12v-.008z" />
                      </svg>
                    </div>
                    <div class="mt-3 text-center sm:mt-0 sm:ml-4 sm:text-left">
                      <h3 class="text-base font-semibold leading-6 text-gray-900" id="modal-title">{{ type === 'buy' ? 'Achat' : 'Vente' }} de {{ selectedItem.symbol }}</h3>
                      <div class="mt-2">
                        <p class="text-sm text-gray-500">
                          <!-- Quantité -->
                          <div class="mt-1">
                            <label for="quantity" class="block text-sm font-medium text-gray-700">Quantité</label>
                            <div class="mt-1 relative rounded-md shadow-sm">
                              <input type="number" step="0.01" name="quantity" id="quantity" class="focus:ring-indigo-500 focus:border-indigo-500 block w-full pl-7 pr-12 sm:text-sm border-gray-300 rounded-md" placeholder="0.00" v-model="quantity">
                              <div class="absolute inset-y-0 right-0 pr-3 flex items-center pointer-events-none">
                                <span class="text-gray-500 sm:text-sm">
                                  {{ selectedItem.symbol }}
                                </span>
                              </div>
                            </div>
                          </div>
                          <!-- Prix unitaire is already filled, it's selectedItem.price -->
                          <div class="mt-1">
                            <label for="unit_price" class="block text-sm font-medium text-gray-700">Prix unitaire</label>
                            <div class="mt-1 relative rounded-md shadow-sm">
                              <input disabled type="number" step="0.01" name="unit_price" id="unit_price" class="focus:ring-indigo-500 focus:border-indigo-500 block w-full pl-7 pr-12 sm:text-sm border-gray-300 rounded-md bg-gray-100" placeholder="0.00" v-model="selectedItem.price">
                              <div class="absolute inset-y-0 right-0 pr-3 flex items-center pointer-events-none">
                                <span class="text-gray-500 sm:text-sm">
                                  €
                                </span>
                              </div>
                            </div>
                          </div>
                          <!-- Prix total -->
                          <div class="mt-1">
                            <label for="total_price" class="block text-sm font-medium text-gray-700">Total</label>
                            <div class="mt-1 relative rounded-md shadow-sm">
                              <span disabled type="number" step="0.01" name="total_price" id="total_price" class="focus:ring-indigo-500 focus:border-indigo-500 block w-full pl-7 pr-12 sm:text-sm border-gray-300 rounded-md bg-gray-100" placeholder="0.00" :value="quantity * selectedItem.price">
                                {{ quantity * selectedItem.price }}
                              </span>
                              <div class="absolute inset-y-0 right-0 pr-3 flex items-center pointer-events-none">
                                <span class="text-gray-500 sm:text-sm">
                                  €
                                </span>
                              </div>
                            </div>
                          </div>
                        </p>
                      </div>
                    </div>
                  </div>
                </div>
                <div class="bg-gray-50 px-4 py-3 sm:flex sm:flex-row-reverse sm:px-6">
                  <button type="button" class="inline-flex w-full justify-center rounded-md border border-transparent bg-cyan-600 px-4 py-2 text-base font-medium text-white shadow-sm hover:bg-red-700 focus:outline-none focus:ring-2 focus:ring-red-500 focus:ring-offset-2 sm:ml-3 sm:w-auto sm:text-sm" @click="doTransaction"> {{ type === 'buy' ? 'Acheter' : 'Vendre' }} </button>
                  <button type="button" class="mt-3 inline-flex w-full justify-center rounded-md border border-gray-300 bg-white px-4 py-2 text-base font-medium text-gray-700 shadow-sm hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2 sm:mt-0 sm:ml-3 sm:w-auto sm:text-sm" @click="modal = false">Cancel</button>
                </div>
              </div>
            </div>
          </div>
        </div>
        <div class="mx-auto max-w-6xl px-4 sm:px-6 lg:px-8">
            <h2 class="text-lg font-medium leading-6 text-gray-900">Achat / Vente</h2>
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
                <th class="bg-gray-50 px-6 py-3 text-right text-sm font-semibold text-gray-900" scope="col">Date de dernière mise à jour</th>
                <th class="bg-gray-50 px-6 py-3 text-right text-sm font-semibold text-gray-900" scope="col">Actions</th>
              </tr>
              </thead>
              <tbody class="divide-y divide-gray-200 bg-white">
              <tr v-for="price in prices" :key="price" class="bg-white">
                <td class="whitespace-nowrap px-6 py-4 text-right text-sm text-gray-500">
                  <p class="text-gray-900 font-medium">{{ price.symbol }}</p>
                </td>
                <td class="whitespace-nowrap px-6 py-4 text-right text-sm text-gray-500">
                  <p class="text-gray-900 font-medium">{{ price.name }}</p>
                </td>
                <td class="whitespace-nowrap px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                  <p class="text-gray-900 font-medium">{{ formattedPrice(price.price) }}</p>
                </td>

                <td class="whitespace-nowrap px-6 py-4 text-right text-sm text-gray-500">
                  <time :datetime="price.last_update">{{ formatAgo(price.last_update) }}</time>
                </td>
                <!-- One bouton "acheter" et un bouton "vendre". Avec un background et des couleurs différentes et des bords arrondis. -->
                <td class="whitespace-nowrap px-6 py-4 text-right text-sm font-medium">
                  <a href="#" class="text-indigo-600 hover:text-indigo-900" @click="openModal(price, 'buy')">Acheter</a>
                  /
                  <a href="#" class="text-indigo-600 hover:text-indigo-900" @click="openModal(price, 'sell')">Vendre</a>
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
        type: null,
        quantity: 0.0,
      }
    },
    mounted() {
      this.getPrices();
    },
    methods: {
      openModal(item, type) {
        this.quantity = 0.0;
        this.selectedItem = item;
        this.modal = true;
        this.type = type;
        console.log(this.selectedItem)
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
            this.getPrices();
          })
          .catch(error => {
            console.log(error);
          });
      },
    },
}
</script>
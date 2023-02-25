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
                      <!-- add svg, if sell or buy -->
                      <svg v-if="type === 'buy'" class="h-6 w-6 text-green-600" x-description="Heroicon name: outline/plus-circle" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor" aria-hidden="true">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6v6m0 0v6m0-6h6m-6 0H6" />
                      </svg>
                      <svg v-if="type === 'sell'" class="h-6 w-6 text-red-600" x-description="Heroicon name: outline/minus-circle" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor" aria-hidden="true">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20 12H4" />
                      </svg>
                    </div>
                    <div class="mt-3 text-center sm:mt-0 sm:ml-4 sm:text-left">
                      <h3 class="text-base font-semibold leading-6 text-gray-900" id="modal-title">{{ type === 'buy' ? 'Achat' : 'Vente' }} de {{ selectedItem.symbol }}</h3>
                      <div class="mt-2">
                        <p class="text-sm text-gray-500">
                          <!-- Quantité -->
                          <div class="mt-1">
                            <label for="quantity" class="block text-sm font-medium text-gray-700">Quantité</label>
                            <div class="grid mt-1 relative rounded-md shadow-sm">
                              <input type="number" step="0.01" min="0" name="quantity" id="quantity" class="focus:ring-indigo-500 focus:border-indigo-500 block w-full pl-7 pr-12 sm:text-sm border-gray-300 rounded-md" placeholder="0.00" v-model="quantity">
                              <div class="absolute inset-y-0 right-0 pr-3 flex items-center pointer-events-none">
                                <span class="text-gray-500 sm:text-sm">
                                  {{ selectedItem.symbol }}
                                </span>
                              </div>
                            </div>

                              <button type="button" class="mt-1 bg-gray-100 hover:bg-gray-200 text-gray-800 font-bold py-2 px-4 rounded-l" @click="quantity = 0">
                                  Minimum
                              </button>
                              <button type="button" class="mt-1 bg-gray-100 hover:bg-gray-200 text-gray-800 font-bold py-2 px-4 rounded-r" @click="quantity = (type === 'buy' ? Math.floor((balance / selectedItem.price)*100)/100 : findBalance(selectedItem.id))">
                                  Maximum
                              </button>
                          </div>
                          <!-- Prix unitaire is already filled, it's selectedItem.price -->
                          <div class="mt-1">
                            <label for="unit_price" class="block text-sm font-medium text-gray-700">Prix unitaire</label>
                            <div class="mt-1 relative rounded-md shadow-sm">
                              <input disabled type="number" step="0.01" name="unit_price" id="unit_price" class="focus:ring-indigo-500 focus:border-indigo-500 block w-full pl-7 pr-12 sm:text-sm border-gray-300 rounded-md bg-gray-100" placeholder="0.00" v-model="selectedItem.price">
                              <div class="absolute inset-y-0 right-0 pr-3 flex items-center pointer-events-none">
                                <span class="text-gray-500 sm:text-sm">
                                  $US
                                </span>
                              </div>
                            </div>
                          </div>
                          <!-- Prix total -->
                          <div class="mt-1">
                            <label for="total_price" class="block text-sm font-medium text-gray-700">Total</label>
                            <div class="mt-1 relative rounded-md shadow-sm">
                              <span disabled type="number" step="0.01" name="total_price" id="total_price" class="focus:ring-indigo-500 focus:border-indigo-500 block w-full pl-7 pr-12 sm:text-sm border-gray-300 rounded-md bg-gray-100" placeholder="0.00" :value="quantity * selectedItem.price">
                                {{ formattedPrice(Math.ceil(quantity * selectedItem.price*100)/100) }}
                              </span>
                            </div>
                          </div>
                          <!-- Disponible -->
                          <div class="mt-1" v-if="type === 'buy'">
                            <label for="available" class="block text-sm font-medium text-gray-700">Disponible</label>
                            <div class="mt-1 relative rounded-md shadow-sm">
                              <!-- If balance > Math.ceil(quantity * selectedItem.price*100)/100) then it should be background green, else red -->
                              <span disabled type="number" step="0.01" name="available" id="available" class="focus:ring-indigo-500 focus:border-indigo-500 block w-full pl-7 pr-12 sm:text-sm border-gray-300 rounded-md" :class="balance >= Math.ceil(quantity * selectedItem.price*100)/100 ? 'bg-green-100' : 'bg-red-100'" placeholder="0.00" :value="balance">
                                {{ formattedPrice(balance) }}
                              </span>
                              <!-- Logo warning if balance < Math.ceil(quantity * selectedItem.price*100)/100) -->
                              <div v-if="balance <= Math.ceil(quantity * selectedItem.price*100)/100" class="absolute inset-y-0 right-0 pr-3 flex items-center pointer-events-none">
                                <svg class="h-5 w-5 text-red-500" x-description="Heroicon name: outline/exclamation-circle" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor" aria-hidden="true">
                                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01M12 5a9 9 0 100 18 9 9 0 000-18z" />
                                </svg>
                              </div>
                            </div>
                          </div>
                          <!-- disponible crypto -->
                          <div class="mt-1" v-if="type === 'sell'">
                            <label for="available" class="block text-sm font-medium text-gray-700">Disponible</label>
                            <div class="mt-1 relative rounded-md shadow-sm">
                              <!-- If balance > Math.ceil(quantity * selectedItem.price*100)/100) then it should be background green, else red -->
                              <span disabled type="number" step="0.01" name="available" id="available" class="focus:ring-indigo-500 focus:border-indigo-500 block w-full pl-7 pr-12 sm:text-sm border-gray-300 rounded-md" :class="quantity <= findBalance(selectedItem.id) ? 'bg-green-100' : 'bg-red-100'" placeholder="0.00" :value="findBalance(selectedItem.id)">
                                {{ findBalance(selectedItem.id) }} {{ selectedItem.symbol }}
                              </span>
                              <!-- Logo warning if balance < Math.ceil(quantity * selectedItem.price*100)/100) -->
                              <div v-if="selectedItem.quantity < quantity" class="absolute inset-y-0 right-0 pr-3 flex items-center pointer-events-none">
                                <svg class="h-5 w-5 text-red-500" x-description="Heroicon name: outline/exclamation-circle" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor" aria-hidden="true">
                                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01M12 5a9 9 0 100 18 9 9 0 000-18z" />
                                </svg>
                              </div>
                            </div>
                          </div>
                        </p>
                      </div>
                    </div>
                  </div>
                </div>
                <div class="bg-gray-50 px-4 py-3 sm:flex sm:flex-row-reverse sm:px-6">
                  <button type="button" :class="type === 'buy' ? 'bg-green-500 hover:bg-green-700' : 'bg-red-500 hover:bg-red-700'" class="w-full inline-flex justify-center rounded-md border border-transparent shadow-sm px-4 py-2 text-base font-medium text-white focus:outline-none focus:ring-2 focus:ring-offset-2 sm:ml-3 sm:w-auto sm:text-sm" @click="doTransaction" :disabled="quantity <= 0"> {{ type === 'buy' ? 'Acheter' : 'Vendre' }} </button>
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
                <th class="bg-gray-50 px-6 py-3 text-right text-sm font-semibold text-gray-900" scope="col">Quantité acquise</th>
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
                  <p class="text-gray-900 font-medium">{{ findBalance(price.id) }}</p>
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
        balance: 0.0,
        type: null,
        quantity: 0.0,
        balanceCrypto: null
      }
    },
    async mounted() {
      await this.getBalanceCrypto();
      await this.getBalance();
      this.getPrices();
    },
    methods: {
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